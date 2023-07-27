import { z } from 'zod';

export const partSchema = z.object({
    part_number: z.string().trim().min(1),
    invoice_number: z.number(),
    distributor_number: z.number()
})

export const invoiceSchema = z.object({
    customer_first_name: z.string().trim().min(3),
    customer_last_name: z.string().trim().min(3),
    customer_address_1: z.string().trim(),
    product_code: z.string().trim().min(8).max(10),
    serial_number: z.number().min(10).max(10),
    model_number: z.string().trim().min(8),
    date_of_purchase: z.date(),
    miles_traveled: z.number().min(1).max(999),
    parts: z.object(partSchema.shape).array()
    // description_of_issue: z.string().trim().min(10).default("")
});  