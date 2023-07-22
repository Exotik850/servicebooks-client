import { z } from 'zod';

export const invoiceSchema = z.object({
    customer_first_name: z.string().trim().min(3).default(""),
    customer_last_name: z.string().trim().min(3).default(""),
    customer_address_1: z.string().trim().default(""),
    product_code: z.string().trim().min(8).max(10).default(""),
    serial_number: z.number().min(10).max(10).default(0),
    model_number: z.string().trim().min(8).default(""),
    date_of_purchase: z.date(),
    description_of_issue: z.string().trim().min(10).default("")
});  