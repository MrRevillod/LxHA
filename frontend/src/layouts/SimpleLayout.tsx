
import { LandingHeader } from "../components/Header"
import { PropsWithChildren } from "react"

export const SimpleLayout = ({ children }: PropsWithChildren) => {

    return (

        <main className="h-screen w-screen flex flex-col px-6 md:px-12 lg:px-28 py-8">

            <LandingHeader />

            <article className="h-full w-full flex flex-row gap-4 items-center">

                {children}

            </article>

        </main>
    )
}