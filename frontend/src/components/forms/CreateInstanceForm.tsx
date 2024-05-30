
import { Input } from "../../components/ui/Input"
import { zodResolver } from "@hookform/resolvers/zod"
import { useUserStore } from "../../store/UserStore"
import { INSTANCETYPE } from "../../lib/types"
import { instanceSchema } from "../../lib/schemas"
import { FieldValues, SubmitHandler, useForm, } from "react-hook-form"

interface CreateInstanceFormProps {
    onClose: () => void
}

export const CreateInstanceForm = ({ onClose }: CreateInstanceFormProps) => {

    const { register, handleSubmit, formState: { errors }, setValue } = useForm({
        resolver: zodResolver(instanceSchema)
    })

    const { createUser } = useUserStore()

    setValue("owner", "carlos")
    const onSubmit = async (formData: any) => {
        // await createUser(formData)
        onClose()
    }

    return (

        <div className="w-[30vw] form ">

            <div className="flex flex-col items-center gap-2">
                <h2 className="text-4xl font-bold text-primary text-center">
                    Create a Instance
                </h2>

                <p className="text-center font-normal text-neutral-900 text-sm">
                    Create a new Instance
                </p>
            </div>

            <form className="flex flex-col gap-4 px-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>
                <div className="flex gap-4">
                    <Input
                        label="Name"
                        type="text"
                        {...register("name")}
                        placeholder="MyInstance"
                        error={errors.name ? (errors.name.message?.toString()) : ""}
                    />
                    <div className="gap-3 flex flex-col w-2/4">
                        <label >Type</label>
                        <select
                            defaultValue={INSTANCETYPE.container}
                            className={`bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
                                        focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5
                                        h-min  `}
                            {...register("type")}>
                            <option value={INSTANCETYPE.container}>Container</option>
                            <option value={INSTANCETYPE.vm}>Virtual Machine</option>
                        </select>
                    </div>
                </div>

                <div className="flex gap-4">
                    <Input
                        label="CPU's"
                        type="number"
                        {...register("cpu", { valueAsNumber: true })}
                        error={errors.cpu ? (errors.cpu.message?.toString()) : ""}
                        value={0}
                    />


                    <Input
                        label="Memory"
                        type="number"
                        {...register("memory", { valueAsNumber: true })}
                        error={errors.cpu ? (errors.cpu.message?.toString()) : ""}
                        value={0}
                    />

                    <Input
                        label="Storage"
                        type="number"
                        {...register("storage", { valueAsNumber: true })}
                        error={errors.storage ? (errors.storage.message?.toString()) : ""}
                        value={0}
                    />
                </div>
                <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                    Create
                </button>
            </form>
        </div>
    )
}