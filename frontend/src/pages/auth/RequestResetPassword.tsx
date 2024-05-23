
import { Spinner } from "../../components/ui/Spinner"
import { MainLayout } from "../../layouts/MainLayout"
import { useHttpStore } from "../../store/HttpStore"
import { ReqResetPassword } from "../../components/forms/ReqResetPaswordForm"

export const ForgotPasswordRequestPage = () => {

    const { isLoading } = useHttpStore()

    return (

        <MainLayout>

            <div className="w-full h-full flex flex-col justify-center items-center">

                {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

                <ReqResetPassword />

            </div>

        </MainLayout>
    )
}
