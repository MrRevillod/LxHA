
// import { For } from "./ui/For"

// interface Props<T> {
//     data: T[],
//     columns: string[],
//     actions: VoidFunction[],
// }

// export const TableRow = <T,>({ data, columns, actions }: Props<T>) => {

//     return (

//         <div className="w-full h-full flex flex-col mt-24 gap-4 text-neutral-950">

//             <div className="w-full flex flex-row justify-between gap-4 py-4">

//                 <For of={columns} render={(column, index) => (
//                     <div key={index} className={`w-full`}>
//                         <h4 className="font-semibold">{column}</h4>
//                     </div>
//                 )} />

//             </div>

//             <hr className="w-full border border-neutral-300 -mt-6" />

//         </div>
//     )
// }