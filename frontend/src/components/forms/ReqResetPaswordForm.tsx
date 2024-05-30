
import { Input } from "../../components/ui/Input"
import { useAuth } from "../../store/AuthContext"
import { zodResolver } from "@hookform/resolvers/zod"
import { emailSchema } from "../../lib/schemas"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

import "../../index.css"

export const ReqResetPassword = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(emailSchema)
    })

    const { useRequestResetPassword } = useAuth()

    const onSubmit = async (formData: any) => {
        await useRequestResetPassword(formData)
    }

    return (

        <div className="-mt-8 form w-4/6 lg:w-3/6 2xl:w-2/6 h-3/4">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-4xl font-bold text-primary text-center">
                    Reset Password
                </h2>

                <p className="text-center font-normal text-neutral-900 text-sm">
                    You will receive an email with instructions to reset your password.
                </p>
            </div>

            <form className="flex flex-col gap-4 px-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>
                <Input
                    label="Email"
                    type="email"
                    {...register('email')}
                    placeholder="john@domain.com"
                    error={errors.email ? (errors.email.message?.toString()) : ""}
                />

                <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                    Send
                </button>
            </form>
        </div>
    )
}