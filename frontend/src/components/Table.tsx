
import { For } from "./ui/For"
import { ReactNode } from "react"
import { UserStore } from "../store/UserStore"
import { toSuspensive } from "../lib/string"
import { InstanceStore } from "../store/InstanceStore"

interface Props {
    dataStore: UserStore | InstanceStore,
    children: ReactNode,
}

export const Table = ({ dataStore, children }: Props) => {

    const { nColumns, columns } = dataStore

    return (

        <div className="w-full flex flex-col gap-8">

            <div className={`w-full grid grid-rows-1 grid-cols-${nColumns} gap-4 py-4 border-b border-neutral-300`}>

                <For of={columns} render={(column: string, _index: number) => (
                    <h4 className="font-semibold">{column}</h4>
                )} />

            </div>

            <div className="w-full h-full flex flex-col -mt-4">
                {children}
            </div>

        </div>
    )
}

interface TableFieldProps {
    value: string | number,
    variant?: "date"
}

export const TableField = ({ value, variant }: TableFieldProps) => {

    const classes = "w-full overflow-hidden overflow-ellipsis whitespace-nowrap text-md"

    if (variant === "date") return <p className={classes}>{new Date(value).toLocaleDateString()}</p>

    return <p className={classes}>{toSuspensive(value)}</p>
}
