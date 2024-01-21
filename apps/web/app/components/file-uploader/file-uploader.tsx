'use client';

import { SubmitHandler, useForm } from 'react-hook-form';
import styles from './file-uploader.module.css'
import { useRef, useState } from 'react';
import { rspc } from '../../../src/lib/rspc';

type FileUploadFormValues =
{
    file: string;
}

export function FileUploader()
{
    const { register, handleSubmit, setError } = useForm<FileUploadFormValues>();

    const [ files, setFiles ] = useState<File[]>([ ]);

    const fileInputRef = useRef<HTMLInputElement>( null );

    const { mutate } = rspc.useMutation(
        [ 'files.upload' ],
        {
            retry: false,
        });

    function onFileChanged( event: React.ChangeEvent<HTMLInputElement> ): void
    {
        const files = Array.from( event.target.files ?? [ ] );

        setFiles( files );
    }

    const onSubmit: SubmitHandler<FileUploadFormValues> = () =>
    {
        if ( files.length <= 0 )
            return setError('file', { message: 'No file found' });

        const file = files[0]!;

        mutate(
            {
                file_name: file.name,

                content_type: file.type,
            },
            {
                onSuccess( data )
                {
                    fetch( data.signed_url,
                    {
                        method: 'PUT',

                        body: file,

                        headers:
                        {
                            'Content-Type': file.type,
                        }
                    })
                },
            }
        );
    }

    return (
        <div className={ styles.fileUploader } >

            {/* UserId { userId } */}

            <form className='flex-1 flex' onSubmit={ handleSubmit( onSubmit ) }>

                <input type='file' { ...register( 'file' ) } ref={ fileInputRef } onChange={ onFileChanged } className={ styles.filterUploaderInput } />

                <div className='flex-1 flex justify-center items-center' onClick={() => fileInputRef.current?.click() }>

                    {
                        files.length <= 0
                        ?
                            'Selecione uma imagem!'
                        :
                            files.map(( f ) => f.type)
                    }

                </div>

                <input type='submit' />

            </form>

        </div>
    )
}
