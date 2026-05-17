import { Header } from "./Header";
export function Page({ children }: { readonly children: React.ReactNode }){
    return (
        <>
            <Header></Header>
            {children}
        </>
    ); 
}