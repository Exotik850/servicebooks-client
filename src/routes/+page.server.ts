import { invoiceSchema } from '$lib/invoiceSchema';
import type { Actions } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

export const actions: Actions = {
  default: async (event) => {
    const formData = Object.fromEntries(await event.request.formData());
    console.log(formData);
    const invoiceData = invoiceSchema.safeParse(formData);
    console.log(invoiceData);

    if (!invoiceData.success) {
        // Loop through the errors array and create a custom errors array
        const errors = invoiceData.error.errors.map((error) => {
          return {
            field: error.path[0],
            message: error.message
          };
        });

        
        
        return {status_code: 400, packet: { error: true, errors }};
      }
  }
};