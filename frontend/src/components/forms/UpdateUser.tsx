
import { Show } from "../ui/Show"
import { Input } from "../ui/Input"
import { useAuth } from "../../store/AuthContext"
import { Spinner } from "../ui/Spinner"
import { zodResolver } from "@hookform/resolvers/zod"
import { useHttpStore } from "../../store/HttpStore"
import { useUserStore } from "../../store/UserStore"
import { profileSchema } from "../../lib/schemas"
import { ROLE, User, FormProfileValues } from "../../lib/types"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

interface Props {
    user: User,
    handleClose: () => void
}

export const UpdateUserForm = ({ user, handleClose }: Props) => {

    const { register, handleSubmit, setError, formState: { errors }, getValues, reset } = useForm({
        resolver: zodResolver(profileSchema)
    })

    const { role } = useAuth()
    const { updateUser, getUsers } = useUserStore()
    const { data, isLoading, resetStore } = useHttpStore()

    const getUpdatedFields = (fields: any) => {

        const values: Partial<FormProfileValues> = {}

        for (const key in fields) {
            if (fields[key] !== user[key as keyof User] && fields[key] !== "") {
                values[key as keyof FormProfileValues] = fields[key]
            }
        }

        return values
    }

    const onSubmit = async () => {

        const fields = getUpdatedFields(getValues())
        const status = await updateUser(user.id, fields)

        if (status === 409 && data.conflicts) {

            Object.entries(data.conflicts).forEach(([key, value]) => {
                setError(key, { type: "manual", message: value?.toString() })
            })

            return
        }

        if (status === 200) {
            resetStore()
            reset()
            handleClose()
        }

        await getUsers()
    }

    const selectClasses = (error: any) => `border-1 ${error ? "border-red-400" : "border-neutral-500"} rounded-lg 
        focus:outline-none  focus:ring-blue-500 focus:border-blue-500 w-full
        pl-4 placeholder-neutral-400 text-neutral-950 bg-white p-2
    `

    return (

        <div className="w-full form">

            <Show when={isLoading}>
                <Spinner classes="fixed z-10 opacity-100" />
            </Show>

            <form className="flex flex-col gap-4 px-8 w-full"
                onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}
            >
                <div className="flex gap-4">

                    <Input
                        label="Name"
                        type="text"
                        {...register("name")}
                        placeholder={user.name}
                        error={errors.name ? (errors.name.message?.toString()) : ""}
                    />
                    <Input
                        label="Username"
                        type="text"
                        {...register("username")}
                        placeholder={user.username}
                        error={errors.name ? (errors.name.message?.toString()) : ""}
                    />
                </div>

                <div className="flex flex-row gap-4 items-center">

                    <div className={`${role === ROLE.ADMINISTRATOR ? "w-1/2" : "w-full"}`}>

                        <Input label="Email"
                            type="email"
                            {...register('email')}
                            placeholder={user.email}
                            error={errors.email ? (errors.email.message?.toString()) : ""}
                        />
                    </div>

                    <Show when={role === ROLE.ADMINISTRATOR}>

                        <div className="flex flex-col gap-3 w-1/2 h-full">
                            <label>Role</label>
                            <select defaultValue={user.role}
                                className={`${selectClasses(errors.role ? (errors.role.message?.toString()) : "")} h-input-px`}
                                {...register("role")}>
                                <option value={ROLE.ADMINISTRATOR}>Administrator</option>
                                <option value={ROLE.USER}>User</option>
                            </select>
                        </div>

                    </Show>
                </div>

                <Input label="Password"
                    type="password"
                    placeholder="●●●●●●●●●●"
                    {...register('password')}
                    error={errors.password ? (errors.password.message?.toString()) : ""}
                />
                <Input label="Confirm password"
                    type="password"
                    placeholder="●●●●●●●●●●"
                    {...register('confirmPassword')}
                    error={errors.confirmPassword ? (errors.confirmPassword.message?.toString()) : ""}
                />

                <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                    Update
                </button>
            </form>
        </div>
    )
}

