import { invoke, isTauri } from '@tauri-apps/api/core';
import { type FormEvent, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { pageLinks, pageName } from './wiki';
import './style.css';

function App() {
  const [notes, setNotes] = useState<Map<string, string> | null>(null);
  const [title, setTitle] = useState<string | null>(null);
  const [nameInput, setNameInput] = useState('');
  const [body, setBody] = useState('');
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    invoke<Record<string, string>>('load_notes').then(
      (saved) => setNotes(new Map(Object.entries(saved))),
      (cause) => setError(`読み込みに失敗しました: ${cause}`),
    );
  }, []);

  if (notes === null) {
    return <main><h1>ぶんぼうぐやさん</h1><p role={error ? 'alert' : 'status'}>{error || '読み込み中…'}</p></main>;
  }

  const savedBody = title === null ? undefined : notes.get(title);
  const dirty = title !== null && body !== (savedBody ?? '');
  const canNavigate = !dirty && !saving;

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
    if (title === null || notes === null || saving) return;
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

  return (
    <main>
      <h1>ぶんぼうぐやさん</h1>
      <div className="workspace">
        <aside aria-label="ページ一覧">
          <form onSubmit={openFromInput}>
            <label htmlFor="page-name">ページ名</label>
            <input id="page-name" value={nameInput} onChange={(event) => setNameInput(event.target.value)} disabled={!canNavigate} required />
            <button disabled={!canNavigate}>ページを開く</button>
          </form>
          <h2>保存済みページ</h2>
          {notes.size === 0 ? <p>まだありません。</p> : (
            <ul>{[...notes.keys()].sort((a, b) => a.localeCompare(b, 'ja')).map((name) => (
              <li key={name}><button type="button" aria-current={title === name ? 'page' : undefined} disabled={!canNavigate} onClick={() => open(name)}>{name}</button></li>
            ))}</ul>
          )}
        </aside>
        <section aria-label="ノート編集">
          {error && <p role="alert">{error}</p>}
          {title === null ? <p>ページ名を入力するか、保存済みページを選んでください。</p> : (
            <>
              <h2>{title}</h2>
              <label htmlFor="note-body">本文</label>
              <textarea id="note-body" value={body} onChange={(event) => setBody(event.target.value)} disabled={saving} />
              <div className="actions">
                <button type="button" onClick={save} disabled={saving || (!dirty && savedBody !== undefined)}>保存</button>
                <button type="button" onClick={() => { setBody(savedBody ?? ''); setError(''); }} disabled={!dirty || saving}>変更を戻す</button>
                <span role="status">{saving ? '保存中…' : dirty || savedBody === undefined ? '未保存' : '保存済み'}</span>
              </div>
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
