
import { For } from "../components/ui/For"
import { User } from "../lib/types"
import { Helmet } from "react-helmet"
import { SearchBar } from "../components/SearchBar"
import { Pagination } from "../components/Pagination"
import { MainLayout } from "../layouts/MainLayout"
import { ActionIcon } from "../components/Actions"
import { useUserStore } from "../store/UserStore"
import { useModalStore } from "../store/ModalStore"
import { Table, TableField } from "../components/Table"
import { useEffect, useMemo } from "react"
import { Show } from "../components/ui/Show"
import { useAppStore } from "../store/AppStore"

export const UsersPage = () => {

    const { setPageTitle } = useAppStore()
    const userStore = useUserStore()
    const { setModal } = useModalStore()

    useEffect(() => { userStore.getUsers() }, [])

    const handleDeleteUser = async (user: User) => {

        const status = await userStore.deleteUser(user.id)
        if (status === 200) userStore.getUsers()

        setModal("confirmAction")
        await userStore.getUsers()
    }

    const handleInfo = (user: User) => {
        setPageTitle(user.name)
    }

    const memoSlice = useMemo(() => userStore.dataSplice, [userStore.dataSplice])

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Users</title>
            </Helmet>

            <div className="w-full flex flex-col justify-between mt-20 gap-8 text-neutral-950 relative">

                <div className="w-full flex flex-row justify-between items-center">

                    <SearchBar dataStore={userStore} variant="users" />

                    <button onClick={() => setModal("newAccount")} className="flex items-center justify-center text-lg w-44 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                        Create User
                    </button>

                </div>
                <Table dataStore={userStore}>
                    <For of={memoSlice} render={(user: User, index) => (

                        <div key={index} className={`w-full grid grid-cols-${userStore?.nColumns} gap-4 pb-4`}>

                            <TableField value={user.id} />
                            <TableField value={user.name} />
                            <TableField value={user.username} />
                            <TableField value={user.email} />
                            <TableField value={user.role} />
                            <TableField value={user.n_instances} />

                            <div className="w-full xl:flex flex-row justify-between hidden">
                                <ActionIcon variant="info" onClick={() => handleInfo(user)} />
                                <ActionIcon variant="delete" onClick={() => setModal("confirmAction", null, "deleteAccount", () => handleDeleteUser(user))} />
                                <ActionIcon variant="edit" onClick={() => { }} />
                                <ActionIcon variant="email" onClick={() => setModal("newMessage", user, "fromAdmin")} />
                            </div>

                            <div className="w-full flex justify-end xl:hidden">
                                <i className="text-black text-2xl bi bi-three-dots-vertical"></i>
                            </div>

                        </div>

                    )} />

                </Table>

                <Pagination dataStore={userStore} />

            </div>

        </MainLayout>
    )
}
