
import { Spinner } from "../../components/ui/Spinner"
import { MainLayout } from "../../layouts/MainLayout"
import { useHttpStore } from "../../store/HttpStore"
import { ReqResetPassword } from "../../components/forms/ReqResetPaswordForm"
import { Helmet } from "react-helmet"

export const ForgotPasswordRequestPage = () => {

    const { isLoading } = useHttpStore()

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Request Password Reset</title>
            </Helmet>

            <div className="w-full h-full flex flex-col justify-center items-center">

                {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

                <ReqResetPassword />

            </div>

        </MainLayout>
    )
}
