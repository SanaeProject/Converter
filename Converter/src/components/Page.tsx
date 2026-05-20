import { Header } from "./Header";
import { type ReactNode } from "react";

export function Page({ children }: { readonly children: ReactNode }){
    return (
        <>
            <Header></Header>
            {children}
        </>
    ); 
}