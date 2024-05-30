
import { For } from "../components/ui/For"
import { Helmet } from "react-helmet"
import { ActionIcon } from "../components/Actions"
import { MainLayout } from "../layouts/MainLayout"
import { useInstanceStore } from "../store/InstanceStore"
import { Table, TableField } from "../components/Table"
import { InstanceStatusIcon } from "../components/ui/Icons"
import { useEffect, useState } from "react"

import { SearchBar } from "../components/SearchBar"
import { Pagination } from "../components/Pagination"
import { ModalLayout } from "../layouts/ModalLayout"
import { CreateInstanceForm } from "../components/forms/CreateInstanceForm"

export const DashboardPage = () => {

    const [isOpen, setIsOpen] = useState<boolean>(false)

    const handleCreateInstance = () => {
        setIsOpen(true)
    }

    const handleClose = () => {
        setIsOpen(false)
    }

    const instanceStore = useInstanceStore()

    useEffect(() => { instanceStore.getInstances() }, [])

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Instances</title>
            </Helmet>

            <div className="w-full flex flex-col justify-between mt-20 gap-8 text-neutral-950 relative">

                <div className="w-full flex flex-row justify-between items-center">

                    <SearchBar dataStore={instanceStore} variant="users" />

                    <button onClick={handleCreateInstance} className="flex items-center justify-center text-lg w-44 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                        Create Instance
                    </button>

                </div>

                <Table dataStore={instanceStore}>

                    <For of={instanceStore.dataSplice} render={(instance, index) => (

                        <div key={index} className={`w-full grid grid-cols-${instanceStore?.nColumns} gap-4 pb-4`}>

                            <InstanceStatusIcon status={instance.status} />
                            <TableField value={instance.name} />
                            <TableField value={instance.specs.cpu} />
                            <TableField value={instance.specs.ram} />
                            <TableField value={instance.type} />
                            <TableField value={instance.ip_addresses[0]} />

                            <div className="w-full xl:flex flex-row justify-between hidden">
                                <ActionIcon variant="play" onClick={() => { }} />
                                <ActionIcon variant="stop" onClick={() => { }} />
                                <ActionIcon variant="reboot" onClick={() => { }} />
                                <ActionIcon variant="settings" onClick={() => { }} />
                            </div>

                            <div className="w-full flex justify-end xl:hidden">
                                <i className="text-black text-2xl bi bi-three-dots-vertical"></i>
                            </div>

                        </div>

                    )} />

                </Table>

                <ModalLayout isOpen={isOpen} onClose={handleClose}>
                    <CreateInstanceForm onClose={handleClose} />
                </ModalLayout>

                <Pagination dataStore={instanceStore} />

            </div>

        </MainLayout>
    )
}