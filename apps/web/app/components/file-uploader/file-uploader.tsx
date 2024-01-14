'use client';

import { useForm } from 'react-hook-form';
import styles from './file-uploader.module.css'
import { useRef, useState } from 'react';
import { rspc } from '../../../src/lib/rspc';

type FileUploadFormValues =
{
    file: string;
}

export function FileUploader()
{
    const { register } = useForm<FileUploadFormValues>();

    const [ files, setFiles ] = useState<File[]>([ ]);

    const fileInputRef = useRef<HTMLInputElement>( null );

    const { data: userId } = rspc.useQuery([ 'users.get' ]);

    function handleFileChanged( event: React.ChangeEvent<HTMLInputElement> ): void
    {
        const files = Array.from( event.target.files ?? [ ] );

        setFiles( files );
    }

    return (
        <div className={ styles.fileUploader } >

            {/* UserId { userId } */}

            <form style={{ flex: 1, display: 'flex' }}>

                <input type='file' { ...register( 'file' ) } ref={ fileInputRef } onChange={ handleFileChanged } className={ styles.filterUploaderInput } />

                {/* <input type='submit' /> */}

                <div style={{ flex: 1, display: 'flex', justifyContent: 'center', alignItems: 'center' }} onClick={() => fileInputRef.current?.click() }>

                    {
                        files.length <= 0
                        ?
                            'Selecione uma imagem!'
                        :
                            files.map(( f ) => f.type)
                    }

                </div>

            </form>

        </div>
    )
}
