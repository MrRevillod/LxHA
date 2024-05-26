
import { For } from "../components/ui/For"
import { Table } from "../components/Table"
import { Helmet } from "react-helmet"
import { useEffect } from "react"
import { MainLayout } from "../layouts/MainLayout"
import { useUserStore } from "../store/UserStore"

export const UsersPage = () => {

    const userStore = useUserStore()

    useEffect(() => { userStore.getUsers() }, [])

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Users</title>
            </Helmet>

            <Table dataStore={userStore} variant="users">

                <For of={userStore.dataSplice} render={(user, index) => (

                    <div key={index} className={`w-full grid grid-cols-${userStore?.nColumns} gap-4 pb-4`}>

                        <p className="w-full">{user.id}</p>
                        <p className="w-full">{user.name}</p>
                        <p className="w-full">{user.username}</p>
                        <p className="w-full">{user.role}</p>
                        <p className="w-full">{user.instances}</p>

                        <div className="w-full flex flex-row gap-2 md:gap-4 lg:gap-8 xl:gap-12">
                            <i className="text-2xl bi bi-info-circle-fill text-primary" title="Details"></i>
                            <i className="text-2xl bi bi-person-x-fill text-red-600" title="Delete account"></i>
                            <i className="text-2xl bi bi-pencil-square text-green-600" title="Edit account"></i>
                            <i className="text-2xl bi bi-envelope-fill text-neutral-500" title="Send email" ></i>
                        </div>

                    </div>

                )} />

            </Table>

        </MainLayout>
    )
}
