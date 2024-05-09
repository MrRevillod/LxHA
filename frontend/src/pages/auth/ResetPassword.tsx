
import { useForm } from "react-hook-form"
import { useEffect } from "react"
import { useParams } from "react-router-dom"
import { useAuthStore } from "../../store/AuthStore"
import { useHttpStore } from "../../store/HttpStore"
import { toast } from "sonner"

export const ResetPasswordPage = () => {

    const { id, token } = useParams()
    const { status, message } = useHttpStore()
    const { register, handleSubmit, reset } = useForm()
    const { useValidateResetPasswordPage, useResetPassword } = useAuthStore()
    
    useEffect(() => {

        (async () => await useValidateResetPasswordPage(id, token))()

    }, [])

    const onSubmit = async (formData: any) => {
        await useResetPassword(id, token, formData)
        toast(message)
        reset()
    }

    return (

        status === 200 ? (

            <div className="bg-black flex flex-col items-center pt-8">
                <h1 className="text-white">RESET PASSWORD PAGE</h1>

                <form onSubmit={handleSubmit(onSubmit)} className="w-full flex flex-col gap-4 p-20">

                    <input type="password" placeholder="password" className="h-8 rounded-lg" {...register("password")} />
                    <input type="password" placeholder="confirm password" className="h-8 rounded-lg" {...register("confirmPassword")} />

                    <button type="submit" className="bg-white h-8">Reset password</button>

                </form>

            </div>

        ) : (

            <div className="bg-black flex flex-col items-center pt-8">
                <h1 className="text-white">Invalid or expired URL</h1>
            </div>
        )
    )
}
