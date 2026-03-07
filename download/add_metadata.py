from pypdf import PdfReader, PdfWriter

pdf_path = "/home/z/my-project/download/Al_Marjaa_Gap_Analysis.pdf"
reader = PdfReader(pdf_path)
writer = PdfWriter()

for page in reader.pages:
    writer.add_page(page)

writer.add_metadata({
    '/Title': 'Al_Marjaa_Gap_Analysis',
    '/Author': 'Z.ai',
    '/Creator': 'Z.ai',
    '/Subject': 'تحليل فجوات لغة المرجع للاعتماد عليها حصرياً'
})

with open(pdf_path, "wb") as output:
    writer.write(output)

print("Metadata added successfully!")
