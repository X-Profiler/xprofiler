import pandas as pd
import numpy as np
import json

def process_data(raw_data):
    if not raw_data:
        return []
    
    df = pd.DataFrame(raw_data)
    
    # 1. Data Cleaning & Deduplication
    # Drop duplicates based on URL
    df = df.drop_duplicates(subset=['url'])
    
    # Ensure types
    df['price'] = pd.to_numeric(df['price'], errors='coerce')
    df['sales'] = pd.to_numeric(df['sales'], errors='coerce').fillna(0)
    df['rating'] = pd.to_numeric(df['rating'], errors='coerce').fillna(0.0)
    
    # Drop items with missing critical info
    df = df.dropna(subset=['price', 'name'])
    
    # 2. Sort by price (low to high)
    df = df.sort_values(by='price', ascending=True)
    
    # 3. Cost-effectiveness (ROI) Score Calculation
    # Simple algorithm: lower price is better, higher sales is better, higher rating is better.
    # We will min-max scale price, sales, and rating to [0, 1]
    
    if len(df) > 1:
        # Avoid division by zero
        price_range = df['price'].max() - df['price'].min()
        price_range = price_range if price_range > 0 else 1
        
        sales_range = df['sales'].max() - df['sales'].min()
        sales_range = sales_range if sales_range > 0 else 1
        
        rating_range = df['rating'].max() - df['rating'].min()
        rating_range = rating_range if rating_range > 0 else 1
        
        norm_price = 1 - (df['price'] - df['price'].min()) / price_range # Inverted because lower price is better
        norm_sales = (df['sales'] - df['sales'].min()) / sales_range
        norm_rating = (df['rating'] - df['rating'].min()) / rating_range
        
        # Weighted score (e.g., Price: 50%, Rating: 30%, Sales: 20%)
        df['roi_score'] = (norm_price * 50) + (norm_rating * 30) + (norm_sales * 20)
    else:
        df['roi_score'] = 100.0
        
    # Format ROI score
    df['roi_score'] = df['roi_score'].round(2)
    
    # Add a tag for "Best Value" (性价比推荐)
    # The item with the highest roi_score gets the tag
    max_score_idx = df['roi_score'].idxmax()
    df['is_recommended'] = False
    if not pd.isna(max_score_idx):
        df.loc[max_score_idx, 'is_recommended'] = True
        
    # Return processed data as list of dicts
    return df.to_dict(orient='records')

if __name__ == "__main__":
    # Test
    with open("mock_data.json", "r", encoding="utf-8") as f:
        data = json.load(f)
    processed = process_data(data)
    print(json.dumps(processed, ensure_ascii=False, indent=2))