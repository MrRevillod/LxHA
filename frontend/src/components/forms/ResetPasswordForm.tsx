
import { Input } from "../ui/Input"
import { useAuth } from "../../store/AuthContext"
import { useParams } from "react-router-dom"
import { zodResolver } from "@hookform/resolvers/zod"
import { passwordSchema } from "../../lib/schemas"
import { RequestResetPasswordData } from "../../lib/types"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

import "../../index.css"

export const ResetPasswordForm = () => {

    const { register, handleSubmit, formState: { errors }, reset } = useForm({
        resolver: zodResolver(passwordSchema)
    })

    const { id, token } = useParams()
    const { useResetPassword } = useAuth()

    const onSubmit = async (formData: RequestResetPasswordData) => {
        await useResetPassword(id, token, formData)
        reset()
    }

    return (

        <div className="w-full h-full flex flex-col items-center justify-center">

            <div className="w-2/6 h-3/4 form">

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Reset Password
                    </h2>

                    <p className="text-center font-normal text-neutral-900 text-sm">
                        After reseting your password, you will be able to login with your new password.
                    </p>
                </div>

                <form className="flex flex-col gap-4 p-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>
                    <Input
                        label="Passoword"
                        type="password"
                        placeholder="●●●●●●●●●●"
                        {...register('password')}
                        error={errors.password ? (errors.password.message?.toString()) : ""}
                    />

                    <Input
                        label="Confirm password"
                        type="password"
                        placeholder="●●●●●●●●●●"
                        {...register('confirmPassword')}
                        error={errors.confirmPassword ? (errors.confirmPassword.message?.toString()) : ""}
                    />

                    <button type="submit"
                        className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4"
                    >
                        Send

                    </button>
                </form>

            </div>
        </div>

    )
}