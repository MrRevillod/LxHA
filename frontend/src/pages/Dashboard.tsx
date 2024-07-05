
import { For } from "../components/ui/For"
import { ROLE } from "../lib/types"
import { Show } from "../components/ui/Show"
import { Helmet } from "react-helmet"
import { useAuth } from "../store/AuthContext"
import { SearchBar } from "../components/SearchBar"
import { Pagination } from "../components/Pagination"
import { ActionIcon } from "../components/Actions"
import { MainLayout } from "../layouts/MainLayout"
import { useModalStore } from "../store/ModalStore"
import { useInstanceStore } from "../store/InstanceStore"
import { Table, TableField } from "../components/Table"
import { useEffect, useMemo } from "react"
import { InstanceStatusIcon } from "../components/ui/Icons"

export const DashboardPage = () => {

    const { user } = useAuth()
    const { role } = useAuth()
    const { setModal } = useModalStore()

    const instanceStore = useInstanceStore()

    useEffect(() => { instanceStore.getInstances() }, [])

    const memoSlice = useMemo(() => instanceStore.dataSplice, [instanceStore.dataSplice])

    const handleStart =   async (id: any) => { await instanceStore.startInstance(id) }
    const handleStop =    async (id: any) => { await instanceStore.stopInstance(id) }
    const handleRestart = async (id: any) => { await instanceStore.restartInstance(id) }
    const handleRebuild = async (id: any) => { await instanceStore.rebuildInstance(id) }

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Instances</title>
            </Helmet>

            <div className="w-full flex flex-col justify-between mt-20 gap-8 text-neutral-950 relative">

                <div className="w-full flex flex-row justify-between items-center">

                    <SearchBar dataStore={instanceStore} variant="instances" />

                    <Show when={role === ROLE.ADMINISTRATOR}>
                        <button onClick={() => setModal("newInstance", user)} className="flex items-center justify-center text-lg w-44 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                            Create Instance
                        </button>
                    </Show>

                    <Show when={role === ROLE.USER}>
                        <button onClick={() => setModal("newMessage", user, "fromUser")} className="flex items-center justify-center text-lg w-52 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                            Request for support
                        </button>
                    </Show>

                </div>

                <Table dataStore={instanceStore}>

                    <For of={memoSlice} render={(instance, index) => (

                        <div key={index} className={`w-full grid grid-cols-${instanceStore?.nColumns} gap-4 pb-4`}>

                            <InstanceStatusIcon status={instance.status} />
                            <TableField value={instance.name} />
                            <TableField value={instance.specs.cpu} />
                            <TableField value={`${instance.specs.memory} MB`} />
                            <TableField value={instance.type} />
                            <TableField value={instance.ip_addresses[0]} />

                            <div className="w-full xl:flex flex-row justify-between hidden">
                                <ActionIcon variant="start" onClick={() => setModal("confirmAction", null, "startInstance", () => handleStart(instance.id))} />
                                <ActionIcon variant="stop" onClick={() => setModal("confirmAction", null, "stopInstance", () => handleStop(instance.id))} />
                                <ActionIcon variant="reboot" onClick={() => setModal("confirmAction", null, "rebootInstance", () => handleRestart(instance.id))} />
                                <ActionIcon variant="rebuild" onClick={() => setModal("confirmAction", null, "rebuildInstance", () => handleRebuild(instance.id))} />


                            </div>

                            <div className="w-full flex justify-end xl:hidden">
                                <i className="text-black text-2xl bi bi-three-dots-vertical"></i>
                            </div>  

                        </div>

                    )} />

                </Table>

                <Pagination dataStore={instanceStore} />

            </div>

        </MainLayout>
    )
}
