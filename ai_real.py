#!/usr/bin/env python3
"""
AI Real Inference - Al-Marjaa Language
Uses llama-cpp-python for real AI inference
"""
import sys
import os
import argparse

def main():
    parser = argparse.ArgumentParser(description='Al-Marjaa AI Inference')
    parser.add_argument('--prompt', type=str, required=True, help='Input prompt')
    parser.add_argument('--model', type=str, default='models/qwen2.5-0.5b-instruct-q4_k_m.gguf', help='Model path')
    parser.add_argument('--max-tokens', type=int, default=128, help='Max tokens')
    parser.add_argument('--temperature', type=float, default=0.7, help='Temperature')
    args = parser.parse_args()
    
    try:
        from llama_cpp import Llama
        
        # Check if model exists
        if not os.path.exists(args.model):
            print(f"Error: Model not found: {args.model}", file=sys.stderr)
            sys.exit(1)
        
        # Load model
        llm = Llama(
            model_path=args.model,
            n_ctx=2048,
            n_threads=4,
            verbose=False
        )
        
        # Format prompt for Qwen
        formatted_prompt = f"""<|im_start|>system
أنت مساعد برمجي عربي متخصص في تحويل النص العربي الطبيعي إلى كود بلغة المرجع.
أعد فقط الكود بدون شرح إضافي.<|im_end|>
<|im_start|>user
{args.prompt}<|im_end|>
<|im_start|>assistant
"""
        
        # Generate
        output = llm(
            formatted_prompt,
            max_tokens=args.max_tokens,
            temperature=args.temperature,
            stop=["<|im_end|>", "```"]
        )
        
        # Print result
        result = output['choices'][0]['text'].strip()
        print(result)
        
    except ImportError:
        print("Error: llama-cpp-python not installed", file=sys.stderr)
        print("Install with: pip install llama-cpp-python", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
