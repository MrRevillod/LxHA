import { useParams } from "react-router-dom"
import { useUserStore } from "../store/UserStore"
import { MainLayout } from "../layouts/MainLayout"
import { instances } from "../lib/data"
import { For } from "../components/ui/For"
import { Instance } from "../lib/types"

export const UserPage = () => {
    const { userId } = useParams()

    const userStore = useUserStore()

    if (userId === "") { throw new Error("id no encontrado") }

    const user = (userStore.getUser(userId || ""))
    return (
        <MainLayout>
            <div className="mt-40">
                <h1>{user.name}</h1>
                <hr />
                <div className="grid grid-cols-2 w-full mt-10">
                    <div>

                        <div className="grid grid-cols-2 w-1/2 gap-6 mt-8  h-min text-xl">

                            <div className="font-bold">Name</div>
                            <div className="">{user.name}</div>

                            <div className="font-bold">Username</div>
                            <div className="">{user.username}</div>

                            <div className="font-bold">Email</div>
                            <div className="">{user.email}</div>

                            <div className="font-bold">Role</div>
                            <div className="">{user.role}</div>

                            <div className="font-bold">N° Instances</div>
                            <div className="">{user.n_instances}</div>
                        </div>
                    </div>
                    <div className="grid w-full  max-h-96 overflow-scroll">
                        <h2 className="w-100 max-h mb-2">Instances</h2>
                        <div className="overflow-scroll grid gap-2">
                            <For of={instances} render={(instance: Instance, _index: number) => (
                                <div className="w-full bg-white p-3 border text-l flex rounded-sm">
                                    <div className="flex-grow -bold">
                                        {instance.name}
                                    </div>
                                    <div className={`${instance.status === "Running" ? "text-green-500" : "text-red-500"} 
                                    
                                    mr-4 font-bold`}>
                                        {instance.status}
                                    </div>
                                    <div className="self-end text-gray-500 font-inter">
                                        {instance.ip_addresses}
                                    </div>
                                </div>

                            )} />
                        </div>
                        <hr className="" />
                    </div>
                </div>
            </div>
        </MainLayout>
    )
}