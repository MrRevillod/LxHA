
import { Input } from "../ui/Input"
import { zodResolver } from "@hookform/resolvers/zod"
import { ModalLayout } from "../../layouts/ModalLayout"
import { INSTANCETYPE, InstanceSpecs, InstanceData, PublicInstanceData } from "../../lib/types"
import { useModalStore } from "../../store/ModalStore"
import { instanceSchema } from "../../lib/schemas"
import { FieldValues, SubmitHandler, useForm, Controller, set } from "react-hook-form"
import { useUserStore } from "../../store/UserStore"
import Select from "react-select";
import { useAuth } from "../../store/AuthContext"
import { useInstanceStore } from "../../store/InstanceStore"



interface Option {
    label: string
    value: string
}

export const NewInstanceModal = () => {

    const { register, handleSubmit, control, formState: { errors }, reset } = useForm({
        resolver: zodResolver(instanceSchema)
    })

    // const { createInstance } = useInstanceStore()


    const { modals, setModal } = useModalStore()
    const { createInstance, getInstances } = useInstanceStore()
    const userStore = useUserStore()

    const options: Option[] = []
    userStore.dataSplice.forEach((user) => {
        options.push({ value: user.id, label: user.name })
    })


    const onSubmit = async (formData: any) => {
        const specs: InstanceSpecs = {
            cpu: formData.cpu || 0,
            memory: formData.memory || 0,
            storage: formData.storage || 0,
        }


        const data: InstanceData = {
            name: formData.name || "myInstance",
            owner: formData.owner,
            type: formData.type,
            config: specs
        }

        const res = await createInstance(data)
        onClose()
    }
    const onClose = () => {
        setModal("newInstance")
        reset()
    }

    return (
        <ModalLayout isOpen={modals.newInstance} onClose={onClose}>

            <div className="w-full form">

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Create a Instance
                    </h2>

                    <p className="text-center font-normal text-neutral-900 text-sm">
                        Create a new Instance
                    </p>
                </div>

                <form className=" px-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}> *
                    {/* <form className=" px-8 w-full" onSubmit={() => { console.log("submit") }}> */}

                    <div className="grid grid-cols-2 gap-3 mb-3">
                        <Input
                            label="Name"
                            type="text"
                            {...register("name")}
                            placeholder="MyInstance"
                            error={errors.name ? (errors.name.message?.toString()) : ""}
                        />
                        <div className="grid gap-3">
                            <div className="grid grid-cols-2">
                                <label className="">Owner</label>
                                <div className="text-red-500"> {errors.owner?.message?.toString()}</div>

                            </div>

                            <Controller
                                control={control}
                                name="owner"
                                render={({ field: { onChange, onBlur, value, ref } }) => (
                                    <Select
                                        onChange={(e: any) => (onChange(e?.value))} // send value to hook form
                                        onBlur={onBlur} //
                                        options={options}

                                    />
                                )}
                            />


                        </div>
                    </div>

                    <div className="grid grid-cols-4 gap-3">

                        <div className="gap-3 flex flex-col ">
                            <label >Type</label>
                            <select
                                defaultValue={INSTANCETYPE.container}
                                className={`bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
                                    focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5
                                    h-min `}
                                {...register("type")}>
                                <option value={INSTANCETYPE.container}>Container</option>
                                <option value={INSTANCETYPE.vm}>Virtual Machine</option>
                            </select>
                        </div>

                        <Input
                            label="CPU's"
                            type="number"
                            defautlValue={0}
                            {...register("cpu", { valueAsNumber: true })}
                        />

                        <Input
                            label="Memory"
                            type="number"
                            defautlValue={0}
                            {...register("memory", { valueAsNumber: true })}
                        />

                        <Input
                            label="Storage"
                            type="number"
                            defautlValue={0}
                            {...register("storage", { valueAsNumber: true })}
                        />


                    </div>

                    <div className="text-red-500">
                        {errors.storage?.message?.toString()} <br />
                        {errors.cpu?.message?.toString()}<br />
                        {errors.memory?.message?.toString()}
                    </div>
                    <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 w-full font-bold mt-4">
                        Create
                    </button>
                </form>
            </div>
        </ModalLayout>

    )
}