
import { Spinner } from "../../components/ui/Spinner"
import { LoginForm } from "../../components/LoginForm"
import { useHttpStore } from "../../store/HttpStore"

export const LoginPage = () => {

    const { isLoading } = useHttpStore()

    return (

        <main className="h-full w-full  flex items-center justify-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <LoginForm />

        </main>
    )
}