import { invoke, isTauri } from '@tauri-apps/api/core';
import { type FormEvent, useEffect, useMemo, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { pageLinks, pageName } from './wiki';
import './style.css';

const comparePageNames = new Intl.Collator('ja', { numeric: true }).compare;

type Workspace = { folder: string; notes: Record<string, string> };
type LoadResult = [boolean, { Ok: Workspace } | { Err: string }];

function App() {
  const [notes, setNotes] = useState<Map<string, string> | null>(null);
  const [title, setTitle] = useState<string | null>(null);
  const [nameInput, setNameInput] = useState('');
  const [body, setBody] = useState('');
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const deleteDialog = useRef<HTMLDialogElement>(null);
  const [folder, setFolder] = useState('');
  const [canChooseFolder, setCanChooseFolder] = useState(false);
  const [choosingFolder, setChoosingFolder] = useState(false);
  const [error, setError] = useState('');
  const pageNames = useMemo(() => (notes ? [...notes.keys()].sort(comparePageNames) : []), [notes]);

  useEffect(() => {
    invoke<LoadResult>('load_notes').then(
      ([canChoose, result]) => {
        setCanChooseFolder(canChoose);
        if ('Ok' in result) {
          setFolder(result.Ok.folder);
          setNotes(new Map(Object.entries(result.Ok.notes)));
        } else {
          setError(`読み込みに失敗しました: ${result.Err}`);
        }
      },
      (cause) => setError(`読み込みに失敗しました: ${cause}`),
    );
  }, []);

  const savedBody = title === null ? undefined : notes?.get(title);
  const dirty = title !== null && body !== (savedBody ?? '');
  const busy = saving || choosingFolder || deleting;
  const canNavigate = !dirty && !busy;

  async function chooseFolder() {
    if (!canNavigate) return;
    setChoosingFolder(true);
    try {
      const workspace = await invoke<Workspace | null>('choose_storage_folder');
      if (workspace === null) return;
      setFolder(workspace.folder);
      setNotes(new Map(Object.entries(workspace.notes)));
      setTitle(null);
      setNameInput('');
      setBody('');
      setError('');
    } catch (cause) {
      setError(`保存フォルダーを変更できませんでした: ${cause}`);
    } finally {
      setChoosingFolder(false);
    }
  }

  const folderControl = canChooseFolder && (
    <div className="actions storage-folder">
      <button type="button" disabled={!canNavigate} onClick={chooseFolder}>保存フォルダーを選択</button>
      {folder && <span>保存先: {folder}</span>}
    </div>
  );

  if (notes === null) {
    return <main><h1>ぶんぼうぐやさん</h1>{folderControl}<p role={error ? 'alert' : 'status'}>{error || '読み込み中…'}</p></main>;
  }

  function open(name: string) {
    if (!canNavigate || notes === null) return;
    setTitle(name);
    setBody(notes.get(name) ?? '');
    setError('');
  }

  function openFromInput(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const name = pageName(nameInput);
    if (name === null) {
      setError('空の名前や、角括弧・改行を含む名前は使用できません。');
      return;
    }
    open(name);
    setNameInput('');
  }

  async function save() {
    if (title === null || notes === null || busy) return;
    setSaving(true);
    setError('');
    try {
      await invoke('save_note', { title, body });
      setNotes(new Map(notes).set(title, body));
    } catch (cause) {
      setError(`保存に失敗しました: ${cause}`);
    } finally {
      setSaving(false);
    }
  }

  async function deleteNote() {
    deleteDialog.current?.close();
    if (title === null || notes === null || savedBody === undefined || !canNavigate) return;
    setDeleting(true);
    setError('');
    try {
      await invoke('delete_note', { title });
      const remaining = new Map(notes);
      remaining.delete(title);
      setNotes(remaining);
      setTitle(null);
      setBody('');
    } catch (cause) {
      setError(`削除に失敗しました: ${cause}`);
    } finally {
      setDeleting(false);
    }
  }

  return (
    <main>
      <h1>ぶんぼうぐやさん</h1>
      {folderControl}
      <div className="workspace">
        <aside aria-label="ページ一覧">
          <form onSubmit={openFromInput}>
            <label htmlFor="page-name">ページ名</label>
            <input id="page-name" value={nameInput} onChange={(event) => setNameInput(event.target.value)} disabled={!canNavigate} required />
            <button disabled={!canNavigate}>ページを開く</button>
          </form>
          <div className="page-explorer">
            <h2>保存済みページ</h2>
            {pageNames.length === 0 ? <p>まだありません。</p> : (
              <>
                <div className="page-explorer-heading" aria-hidden="true">名前</div>
                <ul role="list" aria-label="保存済みページ" className="page-explorer-list">
                  {pageNames.map((name) => (
                    <li key={name}>
                      <button
                        type="button"
                        aria-current={title === name ? 'page' : undefined}
                        disabled={!canNavigate}
                        onClick={() => open(name)}
                      >
                        <svg width="16" height="16" viewBox="0 0 16 16" aria-hidden="true" fill="none" stroke="currentColor" strokeLinejoin="round">
                          <path d="M3.5 1.5h5l4 4v9h-9z M8.5 1.5v4h4 M5.5 8.5h5 M5.5 11h5" />
                        </svg>
                        <span>{name}</span>
                      </button>
                    </li>
                  ))}
                </ul>
              </>
            )}
          </div>
        </aside>
        <section aria-label="ノート編集">
          {error && <p role="alert">{error}</p>}
          {title === null ? <p>ページ名を入力するか、保存済みページを選んでください。</p> : (
            <>
              <h2>{title}</h2>
              <label htmlFor="note-body">本文</label>
              <textarea id="note-body" value={body} onChange={(event) => setBody(event.target.value)} disabled={busy} />
              <div className="actions">
                <button type="button" onClick={save} disabled={busy || (!dirty && savedBody !== undefined)}>保存</button>
                <button type="button" onClick={() => { setBody(savedBody ?? ''); setError(''); }} disabled={!dirty || busy}>変更を戻す</button>
                {savedBody !== undefined && <button type="button" onClick={() => deleteDialog.current?.showModal()} disabled={!canNavigate}>削除</button>}
                <span role="status">{deleting ? '削除中…' : saving ? '保存中…' : dirty || savedBody === undefined ? '未保存' : '保存済み'}</span>
              </div>
              <dialog ref={deleteDialog} aria-labelledby="delete-title">
                <h3 id="delete-title">ノートを削除</h3>
                <p>「{title}」を削除します。この操作は取り消せません。</p>
                <form method="dialog" className="actions">
                  <button autoFocus>キャンセル</button>
                  <button type="button" onClick={deleteNote}>削除する</button>
                </form>
              </dialog>
              <h3>ページリンク</h3>
              <ul>{pageLinks(body).map((name) => (
                <li key={name}><button type="button" disabled={!canNavigate} onClick={() => open(name)}>{name}</button></li>
              ))}</ul>
            </>
          )}
        </section>
      </div>
    </main>
  );
}

createRoot(document.getElementById('root')!).render(
  isTauri() ? <App /> : (
    <main>
      <h1>ぶんぼうぐやさん</h1>
      <p>ブラウザーではノートを開けません。<code>pnpm dev</code> で起動したアプリのウィンドウを使ってください。</p>
    </main>
  ),
);
