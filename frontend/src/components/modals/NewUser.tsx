
import { Show } from "../ui/Show"
import { Input } from "../ui/Input"
import { Spinner } from "../ui/Spinner"
import { ModalLayout } from "../../layouts/ModalLayout"
import { zodResolver } from "@hookform/resolvers/zod"
import { useHttpStore } from "../../store/HttpStore"
import { useUserStore } from "../../store/UserStore"
import { useModalStore } from "../../store/ModalStore"
import { registerSchema } from "../../lib/schemas"
import { RegisterData, ROLE } from "../../lib/types"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

export const NewUserModal = () => {

    const { register, handleSubmit, setError, formState: { errors }, reset } = useForm({
        resolver: zodResolver(registerSchema)
    })

    const { modals, setModal } = useModalStore()
    const { createUser, getUsers } = useUserStore()
    const { data, isLoading, resetStore } = useHttpStore()

    const handleClose = () => {
        setModal("newAccount")
        reset()
    }

    const onSubmit = async (formData: RegisterData) => {

        const status = await createUser(formData)

        if (status === 409 && data.conflicts) {

            Object.entries(data.conflicts).forEach(([key, value]) => {

                setError(key, {
                    type: "manual",
                    message: value?.toString()
                })
            })

            return
        }

        if (status === 200) {
            handleClose()
            resetStore()
        }

        await getUsers()
    }

    return (

        <ModalLayout isOpen={modals.newAccount} onClose={() => handleClose()}>

            <div className="w-full form">

                <Show when={isLoading}>
                    <Spinner classes="fixed z-10 opacity-100" />
                </Show>

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
        </ModalLayout>
    )
}
