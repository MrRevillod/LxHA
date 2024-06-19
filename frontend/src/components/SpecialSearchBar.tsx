
import { UserStore } from "../store/UserStore"
import { MessageStore } from "../store/MessageStore"
import { useEffect, useState } from "react"
import { InstanceStore } from "../store/InstanceStore"
import { User } from "../lib/types"
import { For } from "./ui/For"
import { useAuth } from "../store/AuthContext"
import { Show } from "./ui/Show"

export interface SpecialSearchBarProps {
    variant: string
    dataStore: MessageStore | UserStore | InstanceStore
    memoslice: User[]
    onSelect: () => void
}

export const SpecialSearchBar = ({ dataStore, memoslice, variant,onSelect}: SpecialSearchBarProps) => {

    const { user } = useAuth()
    const [search, setSearch] = useState<string>(user?.name.toString() || "")
    const [focus, setFocus] = useState<boolean>(false)

    const handleClick = (e: React.MouseEvent<HTMLDivElement>, user: User) => {
        setSearch(user.name)
        onSelect()
        setFocus(false)

    }

    const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(e.target.value)
    }

    useEffect(() => { dataStore.filterBySearch(search) }, [search])
    return (
        <>
            <div className="relative flex flex-row items-center m-0">

                <input type="text" placeholder={`Search ${variant}`}
                    className="pl-10 p-2 border border-neutral-300 rounded-md w-full"
                    value={search}
                    onChange={handleSearch}
                    onFocus={() => setFocus(true)}

                />

                <i className="bi bi-person-fill absolute left-2 top-1/2 transform -translate-y-1/2"></i>
            </div>
            <div className=" w-full h-full grid border  rounded-lg rounded-t-none  ">
                <Show when={focus}>
                    <div className="h-full overflow-scroll ">
                    <For of={memoslice} render={(user: User, index) => (
                        <div onClick={(e) => { handleClick(e, user) }} className=" hover:cursor-pointer p-1 border-b   rounded-sm m-1  
                        "> {user.name}
                        </div>
                    )} />
                    </div>

                </Show>

                
                <Show when={!focus}>
                    {""}
                </Show>
            
            </div>
        </>
    )
}