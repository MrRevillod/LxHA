
import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

import { Input } from "./ui/Input"
import { useAuth } from "../store/AuthContext"
import { LoginData } from "../lib/types"

const formSchema = z.object({

    email: z.string()
        .min(1, { message: "El correo electrónico es requerido" })
        .email({ message: "El correo electrónico no es válido" }),

    password: z.string()
        .min(1, { message: "La contraseña es requerida" })
})

export const LoginForm = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(formSchema)
    })

    const { useLogin, useValidatePermissions } = useAuth()

    const onSubmit = async (formData: LoginData) => {
        await useLogin(formData)
    }

    return (

        <div className="flex flex-col justify-center gap-8 px-4 h-full w-full md:w-1/2 lg:w-7/12">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-3xl font-bold text-neutral-100 text-center">
                    Inicia sesión
                </h2>

                <p className="text-center font-light text-neutral-300 text-sm">
                    Disfruta la experiencia completa de Lxd High Av.
                </p>
            </div>

            <form className="flex flex-col gap-4" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>

                <Input
                    label="Correo electrónico"
                    type="email"
                    {...register('email')}
                    placeholder="john@domain.com"
                    error={errors.email ? (errors.email.message?.toString()) : ""}
                />

                <Input
                    label="Contraseña"
                    type="password"
                    {...register('password')}
                    placeholder="●●●●●●●●●●"
                    error={errors.password ? (errors.password.message?.toString()) : ""}
                    islogin={"true"}
                />

                <button
                    type="submit"
                    className="bg-neutral-100 text-neutral-950 rounded-lg p-2 font-bold mt-4"
                >
                    Ingresar
                </button>

            </form>
        </div>
    )
}
