# 📁 مجلد النماذج - Models Directory

هذا المجلد مخصص لوضع نماذج الذكاء الاصطناعي.

---

## 📥 تحميل النموذج

### النموذج الموصى به: Qwen 2.5

```bash
# تحميل النموذج الصغير (469 MB)
wget -O models/qwen2.5-0.5b-instruct-q4_k_m.gguf \
  'https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf'

# أو النموذج المتوسط (1.1 GB)
wget -O models/qwen2.5-1.5b-instruct-q4_k_m.gguf \
  'https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf'
```

---

## 📋 النماذج المدعومة

| النموذج | الحجم | الرابط |
|---------|-------|--------|
| Qwen 2.5 0.5B | 469 MB | [HuggingFace](https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF) |
| Qwen 2.5 1.5B | 1.1 GB | [HuggingFace](https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF) |
| Qwen 2.5 3B | 2.1 GB | [HuggingFace](https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GGUF) |

---

## ⚙️ الإعدادات

بعد تحميل النموذج، يمكنك تشغيله:

```bash
# تشغيل مع النموذج
./target/release/almarjaa --model models/qwen2.5-0.5b-instruct-q4_k_m.gguf
```

---

## 🔗 روابط مفيدة

- [HuggingFace - Qwen Models](https://huggingface.co/Qwen)
- [llama.cpp Documentation](https://github.com/ggerganov/llama.cpp)
