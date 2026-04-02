import sys
import os
from pathlib import Path
from dotenv import load_dotenv
from google import genai
from google.genai import types

# 1. Загружаем .env из папки со скриптом (или из корня)
script_dir = Path(__file__).parent
env_path = script_dir / ".env"
load_dotenv(dotenv_path=env_path)

API_KEY = os.getenv("GEMINI_API_KEY")

def generate_image(prompt, output_path, source_image_path=None):
    if not API_KEY:
        print("❌ Ошибка: Не задан GEMINI_API_KEY")
        sys.exit(1)

    print(f"🍌 Генерирую через Nano Banana 2 (Flash Image Preview)...")
    print(f"Промпт: '{prompt}'")
    
    # Инициализируем клиент (SDK сам подхватит ключ, если он есть в os.environ, но передадим явно)
    client = genai.Client(api_key=API_KEY)
    
    # Мы используем Nano Banana 2, так как он быстрый и поддерживает редактирование
    model_name = "gemini-3-pro-image-preview"
    
    try:
        if source_image_path and os.path.exists(source_image_path):
            print(f"🖼️ Использую исходное изображение: {source_image_path}")
            
            # Image-to-Image (Редактирование/Стиль)
            with open(source_image_path, "rb") as f:
                source_bytes = f.read()
            
            # Формируем контент: текст + картинка
            contents = [
                prompt,
                types.Part.from_bytes(data=source_bytes, mime_type="image/jpeg")
            ]
            
            response = client.models.generate_content(
                model=model_name,
                contents=contents,
            )
        else:
            # Обычная генерация (Text-to-Image)
            response = client.models.generate_content(
                model=model_name,
                contents=prompt,
                config=types.GenerateContentConfig(
                    response_modalities=["IMAGE"],
                    image_config=types.ImageConfig(
                        aspect_ratio="1:1", # Можно поменять на "16:9" для веба
                        image_size="1K"     # Поддерживает 512, 1K, 2K, 4K
                    )
                )
            )
        
        # Сохраняем результат
        for part in response.parts:
            # Игнорируем "мысли" (thoughts) модели, если они есть
            if part.thought:
                continue
            
            if part.inline_data:
                image_bytes = part.inline_data.data
                
                Path(output_path).parent.mkdir(parents=True, exist_ok=True)
                with open(output_path, 'wb') as handler:
                    handler.write(image_bytes)
                
                print(f"✅ Успешно сохранено в: {output_path}")
                return # Выходим после сохранения первого изображения

    except Exception as e:
        print(f"❌ Ошибка API: {e}")
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Использование:")
        print("1. Text-to-Image : python generate_image_gemini.py '<промпт>' '<путь_сохранения>'")
        print("2. Image-to-Image: python generate_image_gemini.py '<промпт>' '<путь_сохранения>' '<путь_к_исходнику>'")
        sys.exit(1)
        
    prompt = sys.argv[1]
    output_path = sys.argv[2]
    source_img = sys.argv[3] if len(sys.argv) > 3 else None
        
    generate_image(prompt, output_path, source_img)