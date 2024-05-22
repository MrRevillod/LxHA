
import { Header } from "../components/Header"
import { Navbar } from "../components/Navbar"
import { PropsWithChildren } from "react"

export const MainLayout = ({ children }: PropsWithChildren) => {

    return (

        <div className="h-screen w-screen bg-neutral-100 flex flex-row">

            <Navbar />

            <main className="h-full w-full flex flex-col px-6 md:px-12 lg:px-28 py-8">

                <Header />

                <div className="w-full h-full">

                    {children}

                </div>

            </main>

        </div>
    )
}