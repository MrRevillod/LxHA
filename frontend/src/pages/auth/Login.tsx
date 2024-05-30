
import { Helmet } from "react-helmet"
import { Spinner } from "../../components/ui/Spinner"
import { LoginForm } from "../../components/forms/LoginForm"
import { MainLayout } from "../../layouts/MainLayout"
import { useHttpStore } from "../../store/HttpStore"

export const LoginPage = () => {

    const { isLoading } = useHttpStore()

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Login</title>
            </Helmet>

            <main className="h-full w-full  flex items-center justify-center">

                {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

                <LoginForm />

            </main>

        </MainLayout>
    )
}