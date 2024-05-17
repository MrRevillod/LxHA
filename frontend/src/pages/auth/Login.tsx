
import { Spinner } from "../../components/ui/Spinner"
import { LoginForm } from "../../components/LoginForm"
import { useHttpStore } from "../../store/HttpStore"

export const LoginPage = () => {

    const { isLoading } = useHttpStore()

    return (

        <main className="h-screen w-screen bg-neutral-950 flex items-center justify-center gap-8">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <article className="w-1/2 h-full lg:flex items-center justify-center hidden auth-bg-img">

                <h1 className="text-5xl font-bold text-neutral-100 text-center">
                    Welcome to Lxd High Av.
                </h1>
            </article>

            <article className="lg:w-1/2 w-full h-full lg:px-0 md:px-0 px-4 flex items-center justify-center">
                <LoginForm />
            </article>

        </main>
    )
}