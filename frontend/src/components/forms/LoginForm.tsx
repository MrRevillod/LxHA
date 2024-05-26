
import { Input } from "../ui/Input"
import { useAuth } from "../../store/AuthContext"
import { LoginData } from "../../lib/types"
import { zodResolver } from "@hookform/resolvers/zod"
import { loginFormSchema } from "../../lib/schemas"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

import "../../index.css"

export const LoginForm = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(loginFormSchema)
    })

    const { useLogin } = useAuth()

    const onSubmit = async (formData: LoginData) => {
        await useLogin(formData)
    }

    return (

        <div className="-mt-8 w-4/6 lg:w-3/6 2xl:w-2/6 h-3/4 form">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-4xl font-bold text-primary text-center">
                    Login
                </h2>

                <p className="text-center font-normal text-neutral-900 text-sm">
                    Enjoy the full experience of Lx High Availability.
                </p>
            </div>

            <form className="w-11/12 flex flex-col gap-4" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>

                <Input label="Email"
                    type="email"
                    {...register('email')}
                    placeholder="john@domain.com"
                    error={errors.email ? (errors.email.message?.toString()) : ""}
                />

                <Input label="Password"
                    type="password"
                    {...register('password')}
                    placeholder="●●●●●●●●●●"
                    error={errors.password ? (errors.password.message?.toString()) : ""}
                    islogin="true"
                />

                <button type="submit"
                    className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4"
                >
                    Login

                </button>

            </form>

        </div>
    )
}
