document.addEventListener("DOMContentLoaded", function () {
  // 該当する要素をすべて取得
  const elements = document.querySelectorAll(".years-since-marker");

  elements.forEach((el) => {
    const dateStr = el.getAttribute("data-start-date");
    if (!dateStr) return;

    // "YYYY-MM-DD" を分解
    const parts = dateStr.split("-");
    if (parts.length !== 3) return;

    const targetYear = parseInt(parts[0], 10);
    const targetMonth = parseInt(parts[1], 10) - 1; // JSの月は0から始まるため-1
    const targetDay = parseInt(parts[2], 10);

    const target = new Date(targetYear, targetMonth, targetDay);
    const now = new Date();

    // 経過年数の計算
    let years = now.getFullYear() - target.getFullYear();

    // 指定月日前であれば1を引く
    if (
      now.getMonth() < target.getMonth() ||
      (now.getMonth() === target.getMonth() && now.getDate() < target.getDate())
    ) {
      years--;
    }

    // 計算結果をHTMLに挿入
    el.textContent = years;
  });
});
