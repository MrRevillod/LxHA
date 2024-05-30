
import { useRef, useEffect } from 'react'

export const useOutsideClick = (callback: () => void) => {

    const ref = useRef<HTMLDivElement | null>(null)

    useEffect(() => {
        const handleClick = (event: MouseEvent) => {
            if (ref.current && !ref.current.contains(event.target as Node)) {
                callback()
            }
        }

        console.log('useOutsideClick')

        document.addEventListener('click', handleClick, true)

        return () => {
            document.removeEventListener('click', handleClick, true)
        }
    }, [callback])

    return ref
}


