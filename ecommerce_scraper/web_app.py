from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
from pydantic import BaseModel
import uvicorn
import os
from scraper import run_scraper

app = FastAPI(title="电商商品价格自动化采集与对比系统")

class SearchRequest(BaseModel):
    keyword: str
    use_mock: bool = True

@app.post("/api/search")
async def search_items(req: SearchRequest):
    try:
        # 调用核心爬虫逻辑
        results = await run_scraper(req.keyword, use_mock=req.use_mock)
        return {"status": "success", "data": results}
    except Exception as e:
        return {"status": "error", "message": str(e)}

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
STATIC_DIR = os.path.join(BASE_DIR, "static")

@app.get("/")
async def root():
    return FileResponse(os.path.join(STATIC_DIR, "index.html"))

# 挂载静态文件目录
app.mount("/", StaticFiles(directory=STATIC_DIR), name="static")

if __name__ == "__main__":
    uvicorn.run("web_app:app", host="0.0.0.0", port=8000, reload=False)