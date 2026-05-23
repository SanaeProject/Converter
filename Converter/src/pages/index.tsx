import { useState, useEffect, type MouseEvent } from "react";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function IndexPage() {
    const [ files, setFiles] = useState<string[]>([]);
    const [ folder, setFolder] = useState<string>('');
    const [ type, setType] = useState<string>('');
    const [ inputTypes, setInputTypes] = useState<string[]>([]);
    const [ outputTypes, setOutputTypes] = useState<string[]>([]);
    const [ progress, setProgress] = useState<number>(0);
    const [ msg, setMsg] = useState<{success:boolean, msg:string}>();
    const [ isConverting, setIsConverting] = useState<boolean>(false);

    // ファイル読み取りハンドラ
    const fileSelectHandler = async ()=>{
        setProgress(0);

        const files = await open({
            multiple : true,
            filters : [
                { name: '画像ファイル', extensions: inputTypes },
                { name: 'すべてのファイル', extensions: ['*'] }
            ]
        });
        Array.isArray(files)
        if (files) {
            setFiles(prevFiles => [...prevFiles, ...files]);
        }
    };

    // 保存先フォルダー
    const folderSelectHandler = async ()=>{
        const folder = await open({
            directory: true
        });
        folder && setFolder(folder);
    };

    // 送信用ハンドラ
    const submitHandler = async (e: MouseEvent)=>{
        setProgress(0);
        setMsg(undefined);
        e.preventDefault();

        if(files.length === 0){
            alert('ファイルが選択されていません。');
            return;
        }
        if(type === ''){
            alert('変換後の形式が選択されていません。');
            return;
        }

        setIsConverting(true);

        let completed = 0;
        let isErrored = false;
        const promises = files.map(async (file)=>{
            await invoke('convert_file', { input: file, convertTo: type, folder}).then(() => {
                completed++;
            }).catch((err) => {
                isErrored = true;
                setMsg({success: false, msg: err});
                setProgress(0);
            });
        });
        await Promise.all(promises);

        if(!isErrored){
            setMsg({success:true, msg:"成功しました"});
            setProgress(100);
        }
        setIsConverting(false);
    };

    // ドラッグアンドドロップ
    useEffect(()=>{
        let unlisten: (() => void) | undefined;
        let isMounted = true;

        (async ()=>{
            const window = getCurrentWindow();
            unlisten = await window.onDragDropEvent(event=>{
                setProgress(0);
                const payload = event.payload;
                if(payload.type === 'drop'){
                    setFiles(prevFiles => [...prevFiles, ...payload.paths]);
                }
            });

            if(!isMounted && unlisten) unlisten();
        })();

        return () => {
            isMounted = false;
            if (unlisten) unlisten();
        };
    },[]);

    // コマンドライン引数を受け取り
    useEffect(()=>{
        invoke<string[]>('fetch_args').then((args: string[])=>{
            setFiles(prevFiles => [...prevFiles, ...args]);
        });
        invoke<string[]>('fetch_can_read_exts').then((types: string[])=>{
            setInputTypes(types);
        });
        invoke<string[]>('fetch_can_write_exts').then((types: string[])=>{
            setOutputTypes(types);
        });
    }, []);

    return (
        <div className="container mt-5">
            <form>
                <div className="mb-3">
                    <label htmlFor="formFileMultiple" className="form-label">ファイルを選択</label>
                    <input className="form-control" type="button" id="formFileMultiple" value="ファイルを選択" onClick={fileSelectHandler} />
                </div>
                <div className="mb-3">
                    <label htmlFor="outputFormat" className="form-label">変換後の形式</label>
                    <select id="outputFormat" className="form-select" onChange={e => setType(e.target.value)} value={type}>
                        <option value="">変換後の形式を選択</option>
                        {outputTypes.map((ext, idx) => (
                            <option key={idx} value={ext}>{ext}</option>
                        ))}
                    </select>
                </div>
                <div className="mb-3">
                    <button className="btn btn-outline-info" data-bs-toggle="collapse" data-bs-target="#setting" onClick={(e)=>{e.preventDefault();}}>詳細設定</button>
                    <div id="setting" className="collapse container">
                        <div className="mt-2 mb-4">
                            <label htmlFor="selectFolder" className="form-label">データ保存場所</label>{folder && <span className="text-truncate" style={{maxWidth: '100px',marginLeft: '5px'}}>{folder}</span>}
                            <input className="form-control" type="button" id="selectFolder" value="フォルダーを選択" onClick={folderSelectHandler} />
                        </div>
                    </div>
                </div>
                <div className="mb-3">
                    <button type="button" className={`btn btn-primary form-control ${isConverting ? "disabled" : ""}`} onClick={(e) => {
                        submitHandler(e);
                    }}>
                        変換
                    </button>
                </div>
            </form>

            <div>
                <div className="progress mb-2">
                    <div className={`progress-bar progress-bar-striped ${progress === 100 ? "" : "progress-bar-animated"}`} style={{ width: `${progress}%` }}></div>
                </div>
                { msg &&
                    <div className={`alert ${msg.success ? "alert-info" : "alert-danger"}`}>
                        { msg.msg }
                    </div>
                }
            </div>

            <hr/>
            <div className="row">
                {files.length > 0 ?
                    Array.from(files).map((file, idx)=>(
                        <div className="col-12 col-md-3 mb-3" key={idx}>
                            <div className="card">
                                <div className="card-img-top d-flex justify-content-center p-1 bg-light">
                                    <img style={{height:'100px', width:'fit-content', overflowX:'auto'}} src={convertFileSrc(file)} alt="" className=""/>
                                </div>
                                <div className="card-body">
                                    <p className="card-text text-nowrap overflow-auto">
                                        {file}
                                    </p>
                                </div>
                                <button className="btn-close" style={{position:'absolute',top:'10px',right:'10px'}}
                                    onClick={()=>setFiles(files.filter(_file=>_file!==file))}
                                ></button>
                            </div>
                        </div>
                    ))
                    :
                    <p>ファイルが選択されていません。</p>
                }
            </div>
        </div>
    );
}