import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
	const [input, setInput] = useState('');
	const [target, setTarget] = useState('sinhala');
	const [result, setResult] = useState('');
	const [loading, setLoading] = useState(false);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setLoading(true);
		try {
			const output = await invoke<string>('transliterate_text', { input, target });
			setResult(output);
		} catch (err) {
			setResult('Error: ' + err);
		}
		setLoading(false);
	};

	return (
		<div style={{ maxWidth: 500, margin: '2rem auto', fontFamily: 'sans-serif' }}>
			<h2>Singlish to Sinhala/Tamil Transliterator</h2>
			<form onSubmit={handleSubmit}>
				<textarea
					value={input}
					onChange={e => setInput(e.target.value)}
					rows={3}
					style={{ width: '100%', fontSize: '1.1rem' }}
					placeholder="Enter Singlish text..."
				/>
				<div style={{ margin: '1rem 0' }}>
					<label>
						<input
							type="radio"
							value="sinhala"
							checked={target === 'sinhala'}
							onChange={() => setTarget('sinhala')}
						/> Sinhala
					</label>
					<label style={{ marginLeft: 20 }}>
						<input
							type="radio"
							value="tamil"
							checked={target === 'tamil'}
							onChange={() => setTarget('tamil')}
						/> Tamil
					</label>
				</div>
				<button type="submit" disabled={loading || !input.trim()}>
					{loading ? 'Transliterating...' : 'Transliterate'}
				</button>
			</form>
			<div style={{ marginTop: '2rem', minHeight: 40, fontSize: '1.3rem' }}>
				{result && <div><b>Result:</b><br />{result}</div>}
			</div>
		</div>
	);
}

export default App;
