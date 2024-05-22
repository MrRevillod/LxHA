
import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

import { Input } from "./ui/Input"
import { useAuth } from "../store/AuthContext"
import { LoginData } from "../lib/types"

import "../index.css"

const formSchema = z.object({

    email: z.string()
        .min(1, { message: "The email adress is required" })
        .email({ message: "Invalid email adress" }),

    password: z.string()
        .min(1, { message: "The password is required" })
})

export const LoginForm = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(formSchema)
    })

    const { useLogin } = useAuth()

    const onSubmit = async (formData: LoginData) => {
        await useLogin(formData)
    }

    return (

        <div className="flex flex-col w-2/6 h-3/4 items-center justify-center gap-8 px-16 border-1 border-neutral-300">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-3xl font-bold text-neutral-950 text-center">
                    Login
                </h2>

                <p className="text-center font-light text-neutral-900 text-sm">
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
                    className="bg-primary text-neutral-100 rounded-lg p-2 h-12 font-bold mt-4"
                >
                    Login

                </button>

            </form>
        </div>
    )
}
