const pptxgen = require('pptxgenjs');
const html2pptx = require('/data/user/skills/pptx/scripts/html2pptx.js');

async function createPresentation() {
    const pptx = new pptxgen();
    pptx.layout = 'LAYOUT_16x9';
    pptx.author = 'AI Assistant';
    pptx.title = 'Project Architecture';

    // Slide 1: Title
    await html2pptx('/workspace/pptx_demo/slide1.html', pptx);

    // Slide 2: Architecture
    await html2pptx('/workspace/pptx_demo/slide2.html', pptx);

    // Slide 3: Tech Stack
    await html2pptx('/workspace/pptx_demo/slide3.html', pptx);

    // Save
    await pptx.writeFile({ fileName: '/workspace/Project_Architecture_Demo.pptx' });
    console.log('Presentation created successfully at /workspace/Project_Architecture_Demo.pptx');
}

createPresentation().catch(console.error);
