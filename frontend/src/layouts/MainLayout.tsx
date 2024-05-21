
import { Navbar } from "../components/Navbar"
import { PropsWithChildren } from "react"

export const MainLayout = ({ children }: PropsWithChildren) => {

    return (

        <div className="h-screen w-screen bg-neutral-950 flex flex-row">

            <Navbar />

            <main className="
                h-full w-11/12 flex-grow items-center justify-center pt-28 pb-12 px-6 md:px-12 lg:px-28 
            ">

                <div className="w-full h-full">

                    {children}

                </div>

            </main>

        </div>
    )
}