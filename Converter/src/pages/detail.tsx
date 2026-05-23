import { Link } from "react-router-dom";

export function DetailPage() {
    return (
        <div className="container mt-5">
            <h1>詳細</h1>
            <div className="card mt-3">
                <div className="card-body">
                    <h5 className="card-title">Converter</h5>
                    <p className="card-text">
                        Converterは、画像ファイルを別の形式に変換するためのアプリケーションです。<br />
                        ドラッグアンドドロップで簡単にファイルを追加でき、コマンドライン引数もサポートしています。
                    </p>
                    <Link to="/" className="btn btn-primary">ホームに戻る</Link>
                </div>
            </div>
            <div className="card mt-3">
                <div className="card-body">
                    <h5 className="card-title">使用ライブラリ</h5>
                    Converterは以下のライブラリを使用しています。
                    <h6>フロントエンド</h6>
                    <ul>
                        <li>React: 19.1.0</li>
                        <li>Bootstrap: 5.3.8</li>
                    </ul>

                    <h6>バックエンド</h6>
                    <ul>
                        <li>Tauri: 2</li>
                        <li>image: 0.25.10</li>
                        <li>img2svg: 0.1.6</li>
                    </ul>
                    <Link to="/" className="btn btn-primary">ホームに戻る</Link>
                </div>
            </div>
            <div className="card mt-3 mb-5">
                <div className="card-body">
                    <h5 className="card-title">GitHub</h5>
                    <p className="card-text">
                        ConverterのソースコードはGitHubで公開しています。<br />
                        バグ報告や機能追加の提案などがあれば、ぜひGitHubのリポジトリにIssueを作成してください。
                    </p>
                    <a href="https://github.com/SanaeProject/Converter" className="btn btn-primary" target="_blank" rel="noopener noreferrer">GitHubリポジトリ</a>
                </div>
            </div>
            <div className="card mt-3 mb-5">
                <div className="card-body">
                    <h5 className="card-title">ライセンス</h5>
                    <p className="card-text">
                        ConverterはMITライセンスの下で公開されています。<br />
                        詳細はLICENSEをご確認ください。
                    </p>
                    <a href="https://github.com/SanaeProject/Converter/blob/main/LICENSE" className="btn btn-primary" target="_blank" rel="noopener noreferrer">LICENSE</a>
                </div>
            </div>
        </div>
    );
}