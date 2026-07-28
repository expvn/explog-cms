document.addEventListener('DOMContentLoaded', () => {
    // Shared state
    let courseData = [];

    // Redirect Register buttons to register.html
    const setupRegisterRedirects = () => {
        document.addEventListener('click', (e) => {
            const btn = e.target.closest('.nav-btn, #register, .cta-box a');
            if (btn) {
                const text = btn.textContent.toLowerCase();
                if (text.includes('đăng ký') || text.includes('tư vấn') || btn.id === 'register' || btn.classList.contains('nav-btn')) {
                    e.preventDefault();
                    window.location.href = 'register.html';
                }
            }
        });
    };

    // Card rendering for Portal page
    const renderCourseCards = (courses) => {
        const root = document.getElementById('courses-grid-root');
        if (!root) return;

        let html = '';
        courses.forEach((c, index) => {
            const cNum = String(index + 1).padStart(2, '0');
            html += `
                <div class="course-card ${c.card_class}">
                    <div class="course-badge-wrap">
                        <span class="course-number">KHÓA ${cNum}</span>
                        <span class="course-badge ${c.badge_class}">${c.badge}</span>
                    </div>
                    <h3>${c.title}</h3>
                    <p class="course-desc">${c.goal}</p>
                    <div class="course-meta">
                        <span>${c.duration}</span>
                        <span>${c.level}</span>
                    </div>
                    <a href="${c.id}.html" class="btn btn-secondary">Xem chi tiết khóa học</a>
                </div>
            `;
        });
        root.innerHTML = html;
    };

    // Detail rendering for Course Detail Page
    const renderCourseDetail = (courses) => {
        const root = document.getElementById('course-detail-root');
        if (!root) return;

        const courseId = root.getAttribute('data-course-id');
        const c = courses.find(item => item.id === courseId);
        if (!c) {
            root.innerHTML = `
                <div style="padding: 6rem 0; text-align: center;">
                    <h2>Không tìm thấy thông tin khóa học</h2>
                    <p style="margin-top: 1rem;"><a href="index.html" class="btn btn-primary">Quay lại Lộ trình</a></p>
                </div>
            `;
            return;
        }

        // Set document title dynamically
        document.title = `${c.proposed_title} - EXP Academy`;

        // Generate Audiences HTML
        let audiencesHtml = '';
        if (c.audiences && c.audiences.length > 0) {
            audiencesHtml = `
                <h4 style="margin-bottom: 0.5rem; color: var(--text-primary);">Khóa học phù hợp với:</h4>
                <ul class="syllabus-list">
                    ${c.audiences.map(a => `<li>${a}</li>`).join('')}
                </ul>
            `;
        }

        // Generate Syllabus Accordion HTML
        let syllabusHtml = '';
        if (c.syllabus && c.syllabus.length > 0) {
            syllabusHtml = `
                <div class="accordion">
                    ${c.syllabus.map((s, idx) => `
                        <div class="accordion-item">
                            <div class="accordion-header">
                                <span class="accordion-title">${s.title}</span>
                                <span class="accordion-icon">▼</span>
                            </div>
                            <div class="accordion-content">
                                <ul class="syllabus-list">
                                    ${s.items.map(item => `<li>${item}</li>`).join('')}
                                </ul>
                            </div>
                        </div>
                    `).join('')}
                </div>
            `;
        }

        // Generate Capstone Project section HTML if present
        let projectHtml = '';
        if (c.project && c.project.title) {
            projectHtml = `
                <section>
                    <h2 class="${c.accent_class}">${c.project.title}</h2>
                    <p style="color: var(--text-secondary); margin-bottom: 1rem;">${c.project.description}</p>
                    ${c.project.items && c.project.items.length > 0 ? `
                        <div style="background: rgba(255,255,255,0.02); border: 1px solid var(--border-color); padding: 1.5rem; border-radius: 12px;">
                            <ul class="syllabus-list" style="margin-left: 1rem;">
                                ${c.project.items.map(pItem => `<li><strong>${pItem}</strong></li>`).join('')}
                            </ul>
                        </div>
                    ` : ''}
                </section>
            `;
        }

        // Determine specific text colors for sidebar
        let feeColor = 'var(--text-primary)';
        if (c.accent_class === 'csharp-accent') feeColor = 'var(--color-csharp)';
        else if (c.accent_class === 'visual-accent') feeColor = 'var(--color-visual)';
        else if (c.accent_class === 'accent-2d') feeColor = 'var(--color-2d)';
        else if (c.accent_class === 'accent-3d') feeColor = 'var(--color-3d)';
        else if (c.accent_class === 'qa-accent') feeColor = 'var(--color-qa)';
        else if (c.accent_class === 'pm-accent') feeColor = 'var(--color-pm)';

        const html = `
            <!-- Breadcrumb / Back button -->
            <div style="margin-top: 2rem;">
                <a href="index.html" class="back-link">← Quay lại Lộ trình khóa học</a>
            </div>

            <!-- Detail Hero Section -->
            <section class="detail-hero">
                <div class="hero-tag" style="display: inline-block; background: rgba(255, 255, 255, 0.05); border: 1px solid var(--border-color); padding: 0.4rem 1rem; border-radius: 50px; font-size: 0.85rem; font-weight: 600; margin-bottom: 1.5rem;">${c.badge.toUpperCase()}</div>
                <h1 style="font-size: 2.5rem; margin-top: 1rem; margin-bottom: 1.5rem;">${c.proposed_title}</h1>
                <p style="font-size: 1.2rem; color: var(--text-secondary); max-width: 800px;">${c.goal}</p>
            </section>

            <!-- Main Detail Grid -->
            <div class="detail-grid">
                <!-- Left Content Area -->
                <div class="detail-content">
                    <!-- Section: Overview / Objectives -->
                    <section>
                        <h2 class="${c.accent_class}">Mục tiêu khóa học</h2>
                        <p style="margin-bottom: 1.5rem; color: var(--text-secondary);">Trang bị năng lực thực hành thực tế, giải quyết bài toán phát triển sản phẩm thực tế:</p>
                        <div class="icon-grid">
                            <div class="icon-card">
                                <div class="icon-box">🧠</div>
                                <div>
                                    <div class="icon-title">Kiến thức nền tảng vững chắc</div>
                                    <div class="icon-desc">Nắm chắc tư duy cốt lõi để tự phát triển thay vì sao chép code.</div>
                                </div>
                            </div>
                            <div class="icon-card">
                                <div class="icon-box">🛠️</div>
                                <div>
                                    <div class="icon-title">Thực hành thực tế chuẩn Studio</div>
                                    <div class="icon-desc">Áp dụng trực tiếp lý thuyết vào việc dựng gameplay và tối ưu hóa mã nguồn.</div>
                                </div>
                            </div>
                            <div class="icon-card">
                                <div class="icon-box">🎮</div>
                                <div>
                                    <div class="icon-title">Hoàn thiện sản phẩm cá nhân</div>
                                    <div class="icon-desc">Tự phát triển các tính năng và đóng gói game chạy độc lập hoàn chỉnh.</div>
                                </div>
                            </div>
                            <div class="icon-card">
                                <div class="icon-box">🚀</div>
                                <div>
                                    <div class="icon-title">Sẵn sàng thử thách nâng cao</div>
                                    <div class="icon-desc">Nền tảng vững vàng giúp dễ dàng chuyển giao sang bất kỳ nhánh học chuyên sâu nào.</div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <!-- Section: Curriculum -->
                    <section>
                        <h2 class="${c.accent_class}">Chương trình học chi tiết</h2>
                        <p style="color: var(--text-secondary); margin-bottom: 1.5rem;">Giáo trình chia thành các đề mục logic, hỗ trợ lý thuyết đi đôi với thực hành tại lớp.</p>
                        ${syllabusHtml}
                    </section>

                    <!-- Section: Project -->
                    ${projectHtml}

                    <!-- Section: Targets / Prerequisites -->
                    <section>
                        <h2 class="${c.accent_class}">Đối tượng học viên & Yêu cầu</h2>
                        <div style="margin-bottom: 1.5rem;">
                            ${audiencesHtml}
                        </div>
                        <div>
                            <h4 style="margin-bottom: 0.5rem; color: var(--text-primary);">Yêu cầu đầu vào:</h4>
                            <p style="color: var(--text-secondary);">${c.requirements}</p>
                        </div>
                    </section>
                </div>

                <!-- Right Sidebar Area -->
                <div class="sidebar-wrap">
                    <div class="sidebar-card">
                        <h3>Thông tin khóa học</h3>
                        
                        <div class="info-row">
                            <span class="info-label">Thời lượng:</span>
                            <span class="info-value">${c.duration}</span>
                        </div>

                        <div class="info-row">
                            <span class="info-label">Hình thức học:</span>
                            <span class="info-value">Online / Offline</span>
                        </div>

                        <div class="info-row">
                            <span class="info-label">Học phí Online:</span>
                            <span class="info-value" style="color: ${feeColor}; font-weight: 700;">${c.tuition_online}</span>
                        </div>

                        <div class="info-row">
                            <span class="info-label">Học phí Offline:</span>
                            <span class="info-value" style="color: ${feeColor}; font-weight: 700;">${c.tuition_offline}</span>
                        </div>

                        <div class="info-row" style="border-top: 1px solid var(--border-color); padding-top: 1rem; margin-top: 1rem;">
                            <span class="info-label">Đầu ra:</span>
                            <span class="info-value" style="text-align: right; font-size: 0.9rem; max-width: 65%; font-weight: 500;">${c.output}</span>
                        </div>

                        <a href="register.html" id="register" class="btn btn-primary">Đăng ký tư vấn ngay</a>
                    </div>
                </div>
            </div>
        `;
        root.innerHTML = html;
    };

    // Load data and run
    const loadCoursesData = async () => {
        try {
            const response = await fetch('courses.json');
            if (!response.ok) {
                throw new Error('Failed to load courses data');
            }
            courseData = await response.json();
            
            // Render index list if exists
            renderCourseCards(courseData);
            
            // Render detail page if exists
            renderCourseDetail(courseData);
            
            // Setup registration redirects
            setupRegisterRedirects();
        } catch (error) {
            console.error('Error loading course data:', error);
        }
    };

    loadCoursesData();

    // Accordion interaction with auto-height layout fix (using Event Delegation)
    document.addEventListener('click', (e) => {
        const header = e.target.closest('.accordion-header');
        if (!header) return;

        const item = header.parentElement;
        const content = item.querySelector('.accordion-content');
        if (!content) return;

        const isActive = item.classList.contains('active');
        
        // Close siblings
        const siblings = item.parentElement.querySelectorAll('.accordion-item');
        siblings.forEach(sibling => {
            if (sibling !== item) {
                sibling.classList.remove('active');
                const siblingContent = sibling.querySelector('.accordion-content');
                if (siblingContent) {
                    siblingContent.style.maxHeight = null;
                }
            }
        });

        if (!isActive) {
            // Open: add active class
            item.classList.add('active');
            // Set explicit height to trigger smooth transition
            const height = content.scrollHeight + 30; // 30px safety buffer
            content.style.maxHeight = height + 'px';
            
            // When transition ends, set to 'none' so it adjusts dynamically to resize or text wrapping
            const onTransitionEnd = (evt) => {
                if (evt.propertyName === 'max-height') {
                    if (item.classList.contains('active')) {
                        content.style.maxHeight = 'none';
                    }
                    content.removeEventListener('transitionend', onTransitionEnd);
                }
            };
            content.addEventListener('transitionend', onTransitionEnd);
        } else {
            // Close: reset to specific height from 'none' to allow slide
            content.style.maxHeight = content.scrollHeight + 'px';
            // Force browser repaint
            content.offsetHeight;
            // Transition to 0
            content.style.maxHeight = '0px';
            item.classList.remove('active');
        }
    });

    // Roadmap interaction (Event Delegation for safety)
    document.addEventListener('mouseover', (e) => {
        const node = e.target.closest('.roadmap-node-g, .roadmap-node');
        if (!node) return;

        const courseId = node.getAttribute('data-course-id');
        if (courseId) {
            const lines = document.querySelectorAll(`.line-${courseId}`);
            lines.forEach(line => {
                line.classList.add('active');
            });
        }
    });

    document.addEventListener('mouseout', (e) => {
        const node = e.target.closest('.roadmap-node-g, .roadmap-node');
        if (!node) return;

        const courseId = node.getAttribute('data-course-id');
        if (courseId) {
            const lines = document.querySelectorAll(`.line-${courseId}`);
            lines.forEach(line => {
                line.classList.remove('active');
            });
        }
    });
});
