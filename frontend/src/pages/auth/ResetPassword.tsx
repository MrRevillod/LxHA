
import { Show } from "../../components/ui/Show"
import { useAuth } from "../../store/AuthContext"
import { Loading } from "../Loading"
import { useParams } from "react-router-dom"
import { MainLayout } from "../../layouts/MainLayout"
import { useHttpStore } from "../../store/HttpStore"
import { ResetPasswordForm } from "../../components/forms/ResetPasswordForm"
import { useEffect, useState } from "react"

export const ForgotPasswordPage = () => {

    const { id, token } = useParams()
    const { status, isLoading } = useHttpStore()
    const [isValidating, setIsValidating] = useState(true)
    const { useValidateResetPasswordPage } = useAuth()

    useEffect(() => {

        const validatePage = async () => {
            setIsValidating(true)
            await useValidateResetPasswordPage(id, token)
            setIsValidating(false)
        }

        validatePage()

    }, [])

    if (isLoading || isValidating) {
        return <Loading />
    }

    return (

        <MainLayout>

            <Show when={status === 200}>
                <ResetPasswordForm />
            </Show>

            <Show when={status !== 200}>
                <div className="w-full h-full flex justify-center items-center">
                    <h1 className="text-5xl text-center font-bold">Invalid or expired link</h1>
                </div>
            </Show>

        </MainLayout>
    )
}