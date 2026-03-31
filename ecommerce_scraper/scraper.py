import argparse
import asyncio
import json
import logging
import os
from playwright.async_api import async_playwright
from data_processor import process_data

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

async def scrape_jd(keyword, limit=10):
    results = []
    logging.info(f"开始尝试从京东抓取关键词: {keyword}")
    try:
        async with async_playwright() as p:
            # 启动浏览器，非无头模式可以增加成功率，但服务器环境强制无头
            browser = await p.chromium.launch(headless=True)
            page = await browser.new_page(
                user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
            )
            # 设置超时
            page.set_default_timeout(10000)
            
            url = f"https://search.jd.com/Search?keyword={keyword}"
            await page.goto(url)
            
            # 等待商品列表加载
            try:
                await page.wait_for_selector('#J_goodsList', timeout=5000)
                items = await page.query_selector_all('#J_goodsList li.gl-item')
                for item in items[:limit]:
                    name_elem = await item.query_selector('.p-name em')
                    price_elem = await item.query_selector('.p-price i')
                    url_elem = await item.query_selector('.p-name a')
                    commit_elem = await item.query_selector('.p-commit a')
                    
                    if name_elem and price_elem:
                        name = await name_elem.inner_text()
                        price_text = await price_elem.inner_text()
                        url_suffix = await url_elem.get_attribute('href')
                        commit_text = await commit_elem.inner_text() if commit_elem else "0"
                        
                        # 简单清洗数据
                        try:
                            price = float(price_text.strip())
                        except:
                            price = 0.0
                            
                        # 销量/评论数转化
                        sales = 0
                        if '万' in commit_text:
                            sales = int(float(commit_text.replace('万', '').replace('+', '')) * 10000)
                        elif '+' in commit_text:
                            sales = int(commit_text.replace('+', ''))
                        
                        full_url = url_suffix if url_suffix.startswith('http') else f"https:{url_suffix}"
                        
                        results.append({
                            "platform": "京东",
                            "name": name.strip(),
                            "price": price,
                            "sales": sales,
                            "rating": 4.8, # 京东搜索页通常不直接展示评分，此处给个默认较高分
                            "url": full_url
                        })
            except Exception as e:
                logging.warning(f"页面未加载出预期元素，可能被风控或需要验证码: {e}")
            
            await browser.close()
    except Exception as e:
        logging.error(f"Playwright 抓取异常: {e}")
        
    return results

def get_mock_data(keyword):
    logging.info(f"使用模拟数据 (Mock) 进行演示，关键词: {keyword}")
    mock_file = os.path.join(os.path.dirname(__file__), 'mock_data.json')
    try:
        with open(mock_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
            # 根据关键词做简单过滤，如果关键词不在名字里，仍然返回一些数据作为演示
            filtered = [d for d in data if keyword.lower() in d['name'].lower()]
            return filtered if filtered else data
    except Exception as e:
        logging.error(f"读取模拟数据失败: {e}")
        return []

async def run_scraper(keyword, use_mock=False):
    raw_data = []
    
    if use_mock:
        raw_data = get_mock_data(keyword)
    else:
        # 尝试真实抓取
        jd_data = await scrape_jd(keyword)
        if not jd_data:
            logging.warning("真实抓取失败或被拦截，自动回退到模拟数据演示模式...")
            jd_data = get_mock_data(keyword)
        raw_data.extend(jd_data)
        
    # 处理数据（清洗、去重、排序、算性价比）
    processed = process_data(raw_data)
    return processed

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="电商价格自动化采集与对比脚本工具")
    parser.add_argument("keyword", help="搜索商品关键词 (如 'iPhone 15')")
    parser.add_argument("--mock", action="store_true", help="强制使用模拟数据(适用于无网络或被反爬封禁的测试)")
    parser.add_argument("--output", help="输出JSON文件路径", default="result.json")
    
    args = parser.parse_args()
    
    logging.info(f"启动爬虫工具，目标关键词: {args.keyword}")
    
    results = asyncio.run(run_scraper(args.keyword, args.mock))
    
    # 打印部分结果表格
    print("\n========== 数据采集完成并处理完毕 ==========")
    print(f"共获取到 {len(results)} 条有效去重数据。按价格由低到高排序：\n")
    print(f"{'平台':<6} | {'价格':<8} | {'销量':<8} | {'性价比评分':<10} | {'推荐':<4} | {'商品名称':<30}")
    print("-" * 80)
    for r in results:
        rec = "★推荐" if r.get('is_recommended') else ""
        # 截断名称以适配终端显示
        name_short = r['name'][:28] + '..' if len(r['name']) > 30 else r['name']
        print(f"{r['platform']:<6} | ￥{r['price']:<7} | {r['sales']:<8} | {r['roi_score']:<10.2f} | {rec:<4} | {name_short}")
        
    print("-" * 80)
    
    # 导出到JSON
    with open(args.output, 'w', encoding='utf-8') as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    logging.info(f"详细对比结果已保存至: {args.output}")