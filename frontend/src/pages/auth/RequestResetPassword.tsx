
import { z } from "zod"
import { Input } from "../../components/ui/Input"
import { useAuth } from "../../store/AuthContext"
import { Spinner } from "../../components/ui/Spinner"
import { zodResolver } from "@hookform/resolvers/zod"
import { useHttpStore } from "../../store/HttpStore"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

const emailSchema = z.object({
    email: z.string()
        .min(1, { message: "El correo electrónico es requerido" })
        .email({ message: "El correo electrónico no es válido" }),
})

export const ForgotPasswordRequestPage = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(emailSchema)
    })

    const { isLoading } = useHttpStore()
    const { useRequestResetPassword } = useAuth()

    const onSubmit = async (formData: any) => {
        await useRequestResetPassword(formData)
    }

    return (

        <div className="w-full h-full flex flex-col justify-start items-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className="w-5/6 md:w-2/3 lg:w-1/3 h-full flex flex-col justify-center items-center -mt-8">
                <h1 className="text-3xl font-bold text-neutral-100 text-center mt-4">
                    Restablecer contraseña
                </h1>
                <form className="flex flex-col gap-4 p-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>
                    <Input
                        label="Correo electrónico"
                        type="email"
                        {...register('email')}
                        placeholder="john@domain.com"
                        error={errors.email ? (errors.email.message?.toString()) : ""}
                    />

                    <button
                        type="submit"
                        className="bg-neutral-100 text-neutral-950 rounded-lg p-2 font-bold mt-4"
                    >
                        Enviar
                    </button>
                </form>
            </div>
        </div>
    )
}
