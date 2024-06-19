
import { Input } from "../ui/Input"
import { SpecialSearchBar } from "../SpecialSearchBar"
import { zodResolver } from "@hookform/resolvers/zod"
import { ModalLayout } from "../../layouts/ModalLayout"
import { INSTANCETYPE, User } from "../../lib/types"
import { useModalStore } from "../../store/ModalStore"
import { instanceSchema } from "../../lib/schemas"
import { FieldValues, SubmitHandler, useForm, } from "react-hook-form"
import { useUserStore } from "../../store/UserStore"
import { useEffect, useMemo } from "react"
import { For } from "../ui/For"


export const NewInstanceModal = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(instanceSchema)
    })


    const { modals, setModal } = useModalStore()

    const userStore = useUserStore()

    const memoSlice = useMemo(() => userStore.dataSplice, [userStore.dataSplice])

    useEffect(() => { userStore.getUsers() }, [])



    const onSubmit = async (formData: any) => { }

    return (

        <ModalLayout isOpen={modals.newInstance} onClose={() => setModal("newInstance")}>

            <div className="w-full form">

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Create a Instance
                    </h2>

                    <p className="text-center font-normal text-neutral-900 text-sm">
                        Create a new Instance
                    </p>
                </div>

                <form className="grid grid-cols-2 gap-4 px-8 w-full" onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}>
                    <div className="flex flex-col gap-4 ">
                        <Input
                            label="Name"
                            type="text"
                            {...register("name")}
                            placeholder="MyInstance"
                            error={errors.name ? (errors.name.message?.toString()) : ""}
                        />
                        <div className="flex flex-row gap-4">

                            <div className="gap-3 flex flex-col w-2/4">
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
                            <div className="w-2/4">
                                <Input
                                    label="CPU's"
                                    type="number"
                                    {...register("cpu", { valueAsNumber: true })}
                                    error={errors.cpu ? (errors.cpu.message?.toString()) : ""}
                                    value={0}
                                />
                            </div>
                        </div>
                        <div className="flex felx-row  gap-4">

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
                    </div>

                    <div className="flex flex-col gap-3 ">
                        <label >Owner</label>
                        <SpecialSearchBar variant={"Users"} onSelect={() =>{}} dataStore={userStore} memoslice={memoSlice} />
                    </div>
                    <div className="grid" style={{ gridColumnStart: "1", gridColumnEnd: "3" }}>
                        <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                            Create
                        </button>
                    </div>
                </form>
            </div>
        </ModalLayout>

    )
}