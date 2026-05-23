import { Link } from "react-router-dom";
export function Header(){
    return (
        <header>
            <nav className="navbar navbar-expand-lg bg-dark bg-gradient navbar-dark">
            <div className="container-fluid">
                <Link className="navbar-brand" to="/">Converter</Link>
                <button className="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                <span className="navbar-toggler-icon"></span>
                </button>
                <div className="collapse navbar-collapse" id="navbarNav">
                <ul className="navbar-nav">
                    <li className="nav-item">
                        <Link className="nav-link active" to="/">ホーム</Link>
                    </li>
                    <li className="nav-item">
                        <Link className="nav-link" to="/detail">詳細</Link>
                    </li>
                </ul>
                </div>
            </div>
            </nav>
        </header>
    );
}