import { after, before, create, first, select } from '../../util'
import eLifeDataProvider from './eLifeDataProvider'

const getPdfUrl = async (id: string, pdfType: string): Promise<string> => {
  const allowedPdfTypes = ['article', 'figures']
  if (!allowedPdfTypes.includes(pdfType)) {
    return ''
  }
  const response = await eLifeDataProvider.query(id, window.fetch)
  if (pdfType === 'figures') {
    return Promise.resolve(response.articleData.figuresPdf)
  }
  return Promise.resolve(response.articleData.pdf)
}

const getArticlePdfUrl = async (id: string): Promise<string> => {
  return getPdfUrl(id, 'article')
}

const getFiguresPdfUrl = async (id: string): Promise<string> => {
  return getPdfUrl(id, 'figures')
}

const getUrl = (type: string, id: string, title = ''): string => {
  switch (type) {
    case 'bibtex':
      return `https://elifesciences.org/articles/${id}.bib`
    case 'ris':
      return `https://elifesciences.org/articles/${id}.ris`
    case 'mendeley':
      return `https://www.mendeley.com/import?doi=10.7554/eLife.${id}`
    case 'readcube':
      return `https://www.readcube.com/articles/10.7554/eLife.${id}`
    case 'papers':
      return `papers2://url/https%3A%2F%2Felifesciences.org%2Farticles%2F${id}?title=${encodeURIComponent(
        title
      )}`
  }
  return ''
}

const addFiguresPdfUrl = (url: string): void => {
  after(
    select('[data-is-download-pdf-link]')[0],
    create('li', null, create('a', { href: url }, 'Figures PDF'))
  )
}

const buildMenu = (
  articleId: string,
  articleTitle: string,
  pdfUrl: string,
  menuId: string
): void => {
  after(
    select(':--references')[0],
    create(
      'section',
      { id: menuId },
      create('h2', null, 'Download links'),
      create('h3', null, 'Downloads'),
      create(
        'ul',
        null,
        create(
          'li',
          null,
          create(
            'a',
            { href: pdfUrl, 'data-is-download-pdf-link': true },
            'Article PDF'
          )
        )
      ),
      create('h3', null, 'Download citations'),
      create(
        'ul',
        null,
        create(
          'li',
          null,
          create('a', { href: `${getUrl('bibtex', articleId)}` }, 'BibTeX')
        ),
        create(
          'li',
          null,
          create('a', { href: `${getUrl('ris', articleId)}` }, 'RIS')
        )
      ),
      create('h3', null, 'Open citations'),
      create(
        'ul',
        null,
        create(
          'li',
          null,
          create('a', { href: `${getUrl('mendeley', articleId)}` }, 'Mendeley')
        ),
        create(
          'li',
          null,
          create('a', { href: `${getUrl('readcube', articleId)}` }, 'ReadCube')
        ),
        create(
          'li',
          null,
          create(
            'a',
            { href: `${getUrl('papers', articleId, articleTitle)}` },
            'Papers'
          )
        )
      )
    )
  )
}

const buildLinkToMenu = (menuId: string): Promise<unknown> => {
  const url = `#${menuId}`
  const articleTitle = first(':--Article > :--title')
  if (articleTitle === null) {
    return Promise.reject(
      new Error("Can't find element to bolt the download link on top of")
    )
  }
  before(
    articleTitle,
    create(
      'div',
      { class: 'download-link-wrapper' },
      create('a', { href: url, class: 'download-link' }, 'Downloads')
    )
  )
  return Promise.resolve()
}

export const build = (articleId: string, articleTitle: string): void => {
  const menuId = 'downloadMenu'
  try {
    getArticlePdfUrl(articleId)
      .then((pdfUri) => buildMenu(articleId, articleTitle, pdfUri, menuId))
      .then(() => getFiguresPdfUrl(articleId))
      .then((figuresPdfUrl: string) => addFiguresPdfUrl(figuresPdfUrl))
      .then(() => buildLinkToMenu(menuId))
      .catch((err: Error) => {
        throw err
      })
  } catch (err) {
    console.error(err)
  }
}
