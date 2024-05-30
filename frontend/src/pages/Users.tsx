
import { For } from "../components/ui/For"
import { api } from "../lib/axios"
import { User } from "../lib/types"
import { Helmet } from "react-helmet"
import { useEffect } from "react"
import { SearchBar } from "../components/SearchBar"
import { Pagination } from "../components/Pagination"
import { MainLayout } from "../layouts/MainLayout"
import { ActionIcon } from "../components/Actions"
import { useUserStore } from "../store/UserStore"
import { Table, TableField } from "../components/Table"

export const UsersPage = () => {

    const userStore = useUserStore()

    useEffect(() => { userStore.getUsers() }, [])

    const handleContact = async (user: User) => {

        console.log("Sending email...")
        console.log("user", user)
        await api.post("/mailer/contact-test")
    }

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Users</title>
            </Helmet>

            <div className="w-full flex flex-col justify-between mt-20 gap-8 text-neutral-950 relative">

                <div className="w-full flex flex-row justify-between items-center">

                    <SearchBar dataStore={userStore} variant="users" />

                    <button className="flex items-center justify-center text-lg w-44 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                        Create User
                    </button>

                </div>

                <Table dataStore={userStore}>

                    <For of={userStore.dataSplice} render={(user, index) => (

                        <div key={index} className={`w-full grid grid-cols-${userStore?.nColumns} gap-4 pb-4`}>

                            <TableField value={user.id} />
                            <TableField value={user.name} />
                            <TableField value={user.username} />
                            <TableField value={user.email} />
                            <TableField value={user.role} />
                            <TableField value={user.n_instances} />

                            <div className="w-full xl:flex flex-row justify-between hidden">
                                <ActionIcon variant="info" onClick={() => { }} />
                                <ActionIcon variant="delete" onClick={() => { }} />
                                <ActionIcon variant="edit" onClick={() => { }} />
                                <ActionIcon variant="email" onClick={() => handleContact(user)} />
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
