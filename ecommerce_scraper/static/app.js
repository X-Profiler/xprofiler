const { createApp } = Vue;

createApp({
    data() {
        return {
            keyword: 'iPhone 15',
            useMock: true,
            loading: false,
            results: [],
            error: null,
            priceChartInstance: null,
            platformChartInstance: null
        }
    },
    mounted() {
        // 初始化加载数据示例
        this.search();
        
        // 监听窗口缩放重绘图表
        window.addEventListener('resize', () => {
            if (this.priceChartInstance) this.priceChartInstance.resize();
            if (this.platformChartInstance) this.platformChartInstance.resize();
        });
    },
    methods: {
        formatSales(sales) {
            if (sales >= 10000) {
                return (sales / 10000).toFixed(1) + '万+';
            }
            return sales + '+';
        },
        async search() {
            if (!this.keyword.trim()) {
                this.error = "请输入商品关键词";
                return;
            }
            
            this.loading = true;
            this.error = null;
            this.results = [];
            
            try {
                const response = await fetch('/api/search', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        keyword: this.keyword,
                        use_mock: this.useMock
                    })
                });
                
                const data = await response.json();
                
                if (data.status === 'success') {
                    this.results = data.data;
                    this.$nextTick(() => {
                        this.renderCharts();
                    });
                } else {
                    this.error = data.message || "获取数据失败";
                }
            } catch (err) {
                this.error = "网络请求异常，请检查后端服务是否启动。";
                console.error(err);
            } finally {
                this.loading = false;
            }
        },
        renderCharts() {
            this.renderPriceChart();
            this.renderPlatformChart();
        },
        renderPriceChart() {
            const chartDom = document.getElementById('priceChart');
            if (!chartDom) return;
            
            if (this.priceChartInstance) {
                this.priceChartInstance.dispose();
            }
            this.priceChartInstance = echarts.init(chartDom);
            
            // 准备气泡图数据 [价格, 销量, 性价比分, 平台, 商品名]
            const scatterData = this.results.map(item => [
                item.price, 
                item.sales, 
                item.roi_score,
                item.platform,
                item.name
            ]);

            const option = {
                tooltip: {
                    formatter: function (params) {
                        return `${params.value[3]}<br/>商品：${params.value[4]}<br/>价格：¥${params.value[0]}<br/>销量：${params.value[1]}<br/>性价比：${params.value[2]}分`;
                    }
                },
                xAxis: {
                    type: 'value',
                    name: '价格 (元)',
                    scale: true
                },
                yAxis: {
                    type: 'value',
                    name: '销量',
                    scale: true
                },
                series: [{
                    name: '商品分布',
                    type: 'scatter',
                    symbolSize: function (data) {
                        // 气泡大小与性价比正相关
                        return Math.max(10, data[2] / 2);
                    },
                    itemStyle: {
                        color: function(params) {
                            const platform = params.value[3];
                            if (platform === '京东') return '#e1251b';
                            if (platform === '淘宝') return '#ff5000';
                            if (platform === '拼多多') return '#e02e24';
                            return '#5470c6';
                        },
                        opacity: 0.7
                    },
                    data: scatterData
                }]
            };
            
            this.priceChartInstance.setOption(option);
        },
        renderPlatformChart() {
            const chartDom = document.getElementById('platformChart');
            if (!chartDom) return;
            
            if (this.platformChartInstance) {
                this.platformChartInstance.dispose();
            }
            this.platformChartInstance = echarts.init(chartDom);
            
            // 计算各平台平均价格
            const platformStats = {};
            this.results.forEach(item => {
                if (!platformStats[item.platform]) {
                    platformStats[item.platform] = { sum: 0, count: 0 };
                }
                platformStats[item.platform].sum += item.price;
                platformStats[item.platform].count += 1;
            });
            
            const categories = Object.keys(platformStats);
            const avgPrices = categories.map(cat => 
                (platformStats[cat].sum / platformStats[cat].count).toFixed(2)
            );

            const option = {
                tooltip: {
                    trigger: 'axis',
                    axisPointer: { type: 'shadow' }
                },
                xAxis: {
                    type: 'category',
                    data: categories,
                    axisLabel: { interval: 0 }
                },
                yAxis: {
                    type: 'value',
                    name: '平均价格 (元)'
                },
                series: [{
                    data: avgPrices,
                    type: 'bar',
                    barWidth: '40%',
                    itemStyle: {
                        color: function(params) {
                            const platform = params.name;
                            if (platform === '京东') return '#e1251b';
                            if (platform === '淘宝') return '#ff5000';
                            if (platform === '拼多多') return '#e02e24';
                            return '#5470c6';
                        },
                        borderRadius: [4, 4, 0, 0]
                    },
                    label: {
                        show: true,
                        position: 'top',
                        formatter: '¥{c}'
                    }
                }]
            };
            
            this.platformChartInstance.setOption(option);
        }
    }
}).mount('#app');