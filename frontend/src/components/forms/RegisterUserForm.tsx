
import { Input } from "../../components/ui/Input"
import { zodResolver } from "@hookform/resolvers/zod"
import { useUserStore } from "../../store/UserStore"
import { useHttpStore } from "../../store/HttpStore"
import { registerSchema } from "../../lib/schemas"
import { RegisterData, ROLE } from "../../lib/types"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

export const RegisterUserForm = () => {

    const { register, handleSubmit, setError, formState: { errors }, reset } = useForm({
        resolver: zodResolver(registerSchema)
    })

    const { createUser } = useUserStore()
    const { status, data } = useHttpStore()

    const onSubmit = async (formData: RegisterData) => {

        await createUser(formData)

        if (status === 409) {

            if (data.conflicts) {

                Object.entries(data.conflicts).forEach(([key, value]) => {

                    setError(key, {
                        type: "manual",
                        message: value?.toString()
                    })
                })
            }

            return
        }

        reset()
    }

    return (

        <div className="w-[50vw] form">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-4xl font-bold text-primary text-center">
                    Register a user
                </h2>

                <p className="text-center font-normal text-neutral-900 text-sm">
                    Create a new user
                </p>
            </div>

            <form className="flex flex-col gap-4 px-8 w-full"
                onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}
            >
                <div className="flex gap-4">

                    <Input
                        label="Name"
                        type="text"
                        {...register("name")}
                        placeholder="John Doe"
                        error={errors.name ? (errors.name.message?.toString()) : ""}
                    />
                    <Input
                        label="Username"
                        type="text"
                        {...register("username")}
                        placeholder="John Doe"
                        error={errors.username ? (errors.username.message?.toString()) : ""}
                    />

                </div>

                <Input
                    label="Email"
                    type="email"
                    {...register('email')}
                    placeholder="john@domain.com"
                    error={errors.email ? (errors.email.message?.toString()) : ""}
                />

                <select
                    defaultValue={ROLE.USER}
                    className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
                    {...register("role")}>
                    <option value={ROLE.ADMINISTRATOR}>Administrator</option>
                    <option value={ROLE.USER}>User</option>
                </select>

                <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                    Register
                </button>
            </form>
        </div>
    )
}